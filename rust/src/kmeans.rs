use crate::geom;
use rand::distributions::uniform::{UniformFloat, UniformSampler};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::{SeedableRng, StdRng};
use std::f32;

#[allow(clippy::cast_precision_loss)]
fn average_f32(xs: &[f32]) -> f32 {
    (xs.iter().sum::<f32>()) / (xs.len() as f32)
}

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
                label: None,
            });
        }
    }
    centroids
}

/* https://en.wikipedia.org/wiki/K-means%2B%2B */
fn centroids_plus_plus(
    points: &[geom::Point],
    k: usize,
    seed: u64,
) -> Vec<geom::Point> {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    let n: usize = points.len();
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
                    let candidate: f32 = geom::distance(&points[j], &centroid);
                    if candidate < distance {
                        distance = candidate
                    }
                }
                weights[j] = distance;
            }
        }
        centroids.push(
            points[WeightedIndex::new(&weights).unwrap().sample(&mut rng)],
        );
    }
    centroids
}

fn label_points(points: &mut Vec<geom::Point>, centroids: &[geom::Point]) {
    for point in points {
        let mut distance: f32 = f32::MAX;
        for (i, centroid) in centroids.iter().enumerate() {
            let centroid_distance: f32 = geom::distance(point, centroid);
            if centroid_distance < distance {
                point.label = Some(i);
                distance = centroid_distance;
            }
        }
    }
}

fn update_centroids(
    points: &[geom::Point],
    centroids: &mut Vec<geom::Point>,
    k: usize,
) -> f32 {
    let mut delta: f32 = 0.0;
    if 0 < k {
        let n: usize = points.len();
        let mut cohorts: Vec<(Vec<f32>, Vec<f32>)> = Vec::with_capacity(k);
        for _ in 0..k {
            cohorts.push((Vec::with_capacity(n), Vec::with_capacity(n)));
        }
        for point in points {
            if let Some(i) = point.label {
                cohorts[i].0.push(point.x);
                cohorts[i].1.push(point.y);
            }
        }
        for i in 0..k {
            if !cohorts[i].0.is_empty() {
                let update: geom::Point = geom::Point {
                    x: average_f32(&cohorts[i].0),
                    y: average_f32(&cohorts[i].1),
                    label: None,
                };
                delta += geom::distance(&centroids[i], &update);
                centroids[i] = update;
            }
        }
    }
    delta
}

pub fn cluster(
    points: &mut Vec<geom::Point>,
    k: usize,
    threshold: f32,
    seed: u64,
) -> Vec<geom::Point> {
    let mut centroids: Vec<geom::Point> =
        centroids_plus_plus(&points, k, seed);
    let mut i: usize = 0;
    loop {
        label_points(points, &centroids);
        if update_centroids(&points, &mut centroids, k) < threshold {
            break;
        } else {
            i += 1;
        }
    }
    eprintln!("# of iterations: {}", i);
    centroids
}
