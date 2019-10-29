use crate::geom;
use crate::math;
use rand::distributions::uniform::{UniformFloat, UniformSampler};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::{SeedableRng, StdRng};
use std::f32;

#[allow(dead_code)]
fn centroids(bounds: &geom::Bounds, k: usize, seed: u64) -> Vec<geom::Point> {
    let mut centroids: Vec<geom::Point> = Vec::with_capacity(k);
    if 0 < k {
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        let x_uniform: UniformFloat<f32> =
            UniformFloat::<f32>::new(bounds.min_x, bounds.max_x);
        let y_uniform: UniformFloat<f32> =
            UniformFloat::<f32>::new(bounds.min_y, bounds.max_y);
        for _ in 0..k {
            centroids.push(geom::Point {
                x: x_uniform.sample(&mut rng),
                y: y_uniform.sample(&mut rng),
            });
        }
    }
    centroids
}

/* https://en.wikipedia.org/wiki/K-means%2B%2B */
fn centroids_plus_plus(
    xs: &[f32],
    ys: &[f32],
    n: usize,
    k: usize,
    seed: u64,
) -> Vec<geom::Point> {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    let mut centroids: Vec<geom::Point> = Vec::with_capacity(k);
    let mut weights: Vec<f32> = Vec::with_capacity(n);
    for i in 0..k {
        if i == 0 {
            for _ in 0..n {
                weights.push(1.0);
            }
        } else {
            for j in 0..n {
                let mut distance: f32 = f32::MAX;
                for centroid in &centroids {
                    let candidate: f32 = geom::distance(
                        geom::Point { x: xs[j], y: ys[j] },
                        *centroid,
                    ).powi(2);
                    if candidate < distance {
                        distance = candidate
                    }
                }
                weights[j] = distance;
            }
        }
        let index: usize =
            WeightedIndex::new(&weights).unwrap().sample(&mut rng);
        centroids.push(geom::Point {
            x: xs[index],
            y: ys[index],
        });
    }
    centroids
}

fn label_points(
    xs: &[f32],
    ys: &[f32],
    labels: &mut Vec<usize>,
    n: usize,
    centroids: &[geom::Point],
) {
    for i in 0..n {
        let mut distance: f32 = f32::MAX;
        for (j, centroid) in centroids.iter().enumerate() {
            let centroid_distance: f32 =
                geom::distance(geom::Point { x: xs[i], y: ys[i] }, *centroid);
            if centroid_distance < distance {
                labels[i] = j;
                distance = centroid_distance;
            }
        }
    }
}

fn update_centroids(
    xs: &[f32],
    ys: &[f32],
    labels: &[usize],
    n: usize,
    centroids: &mut Vec<geom::Point>,
    k: usize,
) -> f32 {
    let mut delta: f32 = 0.0;
    if 0 < k {
        let mut x_cohorts: Vec<Vec<f32>> = Vec::with_capacity(k);
        let mut y_cohorts: Vec<Vec<f32>> = Vec::with_capacity(k);
        for _ in 0..k {
            x_cohorts.push(Vec::with_capacity(n));
            y_cohorts.push(Vec::with_capacity(n));
        }
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
                delta += geom::distance(centroids[i], update);
                centroids[i] = update;
            }
        }
    }
    delta
}

pub fn cluster(
    xs: &[f32],
    ys: &[f32],
    n: usize,
    k: usize,
    threshold: f32,
    seed: u64,
) -> (Vec<usize>, u16, f32) {
    let mut labels: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        labels.push(0);
    }
    let mut centroids: Vec<geom::Point> =
        centroids_plus_plus(xs, ys, n, k, seed);
    let mut iterations: u16 = 0;
    loop {
        label_points(xs, ys, &mut labels, n, &centroids);
        if update_centroids(xs, ys, &labels, n, &mut centroids, k) < threshold
        {
            break;
        } else {
            iterations += 1;
        }
    }
    let mut error: f32 = 0.0;
    for i in 0..n {
        let distance: f32 = geom::distance(
            geom::Point { x: xs[i], y: ys[i] },
            centroids[labels[i]],
        );
        error += distance * distance;
    }
    (labels, iterations, error)
}
