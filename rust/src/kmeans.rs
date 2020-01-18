use crate::geom;
use crate::math;
use rand::distributions::uniform::{UniformFloat, UniformSampler};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::{SeedableRng, StdRng};
use std::f32;

#[allow(dead_code)]
fn centroids(
    bounds: &geom::Bounds,
    k: usize,
    rng: &mut StdRng,
) -> Vec<geom::Point> {
    let mut centroids: Vec<geom::Point> = Vec::with_capacity(k);
    if 0 < k {
        let x_uniform: UniformFloat<f32> =
            UniformFloat::<f32>::new(bounds.min_x, bounds.max_x);
        let y_uniform: UniformFloat<f32> =
            UniformFloat::<f32>::new(bounds.min_y, bounds.max_y);
        for _ in 0..k {
            centroids.push(geom::Point {
                x: x_uniform.sample(rng),
                y: y_uniform.sample(rng),
            });
        }
    }
    centroids
}

/* NOTE: https://en.wikipedia.org/wiki/K-means%2B%2B */
fn centroids_plus_plus(
    xs: &[f32], /* NOTE: xs.len() == n */
    ys: &[f32], /* NOTE: ys.len() == n */
    n: usize,
    k: usize,
    rng: &mut StdRng,
) -> Vec<geom::Point> {
    let mut centroids: Vec<geom::Point> = Vec::with_capacity(k);
    let mut weights: Vec<f32> = vec![1.0; n];
    macro_rules! random_centroid {
        () => {
            let index: usize =
                WeightedIndex::new(&weights).unwrap().sample(rng);
            centroids.push(geom::Point {
                x: xs[index],
                y: ys[index],
            });
        };
    }
    random_centroid!();
    for _ in 1..k {
        for j in 0..n {
            let mut distance: f32 = f32::MAX;
            for centroid in &centroids {
                let candidate: f32 = geom::distance_f32(
                    geom::Point { x: xs[j], y: ys[j] },
                    *centroid,
                )
                .powi(2);
                if candidate < distance {
                    distance = candidate
                }
            }
            weights[j] = distance;
        }
        random_centroid!();
    }
    centroids
}

fn label_points(
    xs: &[f32],              /* NOTE: xs.len() == n */
    ys: &[f32],              /* NOTE: ys.len() == n */
    labels: &mut Vec<usize>, /* NOTE: labels.len() == n */
    n: usize,
    centroids: &[geom::Point],
) {
    for i in 0..n {
        let mut distance: f32 = f32::MAX;
        for (j, centroid) in centroids.iter().enumerate() {
            let centroid_distance: f32 = geom::distance_f32(
                geom::Point { x: xs[i], y: ys[i] },
                *centroid,
            );
            if centroid_distance < distance {
                labels[i] = j;
                distance = centroid_distance;
            }
        }
    }
}

fn update_centroids(
    xs: &[f32],       /* NOTE: xs.len() == n */
    ys: &[f32],       /* NOTE: ys.len() == n */
    labels: &[usize], /* NOTE: labels.len() == n */
    n: usize,
    centroids: &mut Vec<geom::Point>,
    k: usize,
) -> f32 {
    let mut delta: f32 = 0.0;
    if 0 < k {
        let mut x_cohorts: Vec<Vec<f32>> = vec![Vec::with_capacity(n); k];
        let mut y_cohorts: Vec<Vec<f32>> = vec![Vec::with_capacity(n); k];
        for i in 0..n {
            x_cohorts[labels[i]].push(xs[i]);
            y_cohorts[labels[i]].push(ys[i]);
        }
        for i in 0..k {
            if let (Some(x), Some(y)) = (
                math::average_f32(&x_cohorts[i]),
                math::average_f32(&y_cohorts[i]),
            ) {
                let update: geom::Point = geom::Point { x, y };
                delta += geom::distance_f32(centroids[i], update);
                centroids[i] = update;
            }
        }
    }
    delta
}

#[allow(clippy::many_single_char_names)]
pub fn cluster(
    xs: &[f32],
    ys: &[f32],
    k: usize,
    threshold: f32,
    loops: usize,
    seed: u64,
) -> Option<(Vec<usize>, usize, u16, f32)> {
    let n: usize = xs.len();
    if (n == 0) || (n != ys.len()) || (k == 0) || (threshold <= 0.0) {
        return None;
    }
    let mut labels: Vec<usize> = vec![0; n];
    let mut iterations: u16 = 0;
    let mut error: f32 = f32::MAX;
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    for _ in 0..loops {
        let mut centroids: Vec<geom::Point> =
            centroids_plus_plus(xs, ys, n, k, &mut rng);
        let mut i: u16 = 0;
        let mut l: Vec<usize> = vec![0; n];
        loop {
            label_points(xs, ys, &mut l, n, &centroids);
            if update_centroids(xs, ys, &l, n, &mut centroids, k) < threshold {
                break;
            } else {
                i += 1;
            }
        }
        let mut e: f32 = 0.0;
        for i in 0..n {
            e += geom::distance_f32(
                geom::Point { x: xs[i], y: ys[i] },
                centroids[l[i]],
            )
            .powi(2);
        }
        if e < error {
            labels = l;
            iterations = i;
            error = e;
        }
    }
    Some((labels, n, iterations, error))
}
