use crate::geom;
use rand::distributions::uniform::{UniformFloat, UniformSampler};
use rand::prelude::{SeedableRng, StdRng};
use std::f32;

fn centroids(bounds: &geom::Bounds, k: usize, seed: u64) -> Vec<geom::Point> {
    if 0 < k {
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        let x_uniform = UniformFloat::<f32>::new(bounds.min_x, bounds.max_x);
        let y_uniform = UniformFloat::<f32>::new(bounds.min_y, bounds.max_y);
        let mut centroids: Vec<geom::Point> = Vec::with_capacity(k);
        for _ in 0..k {
            let x: f32 = x_uniform.sample(&mut rng);
            let y: f32 = y_uniform.sample(&mut rng);
            centroids.push(geom::Point { x, y, label: None });
        }
        return centroids;
    }
    Vec::new()
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

#[allow(clippy::cast_precision_loss)]
fn average_f32(xs: &[f32]) -> f32 {
    (xs.iter().sum::<f32>()) / (xs.len() as f32)
}

fn adjust_centroids(
    points: &[geom::Point],
    centroids: &mut Vec<geom::Point>,
    k: usize,
) {
    if 0 < k {
        let mut cohorts: Vec<(Vec<f32>, Vec<f32>)> = Vec::with_capacity(k);
        for _ in 0..k {
            cohorts.push((Vec::new(), Vec::new()));
        }
        for point in points {
            if let Some(i) = point.label {
                cohorts[i].0.push(point.x);
                cohorts[i].1.push(point.y);
            }
        }
        for i in 0..k {
            if !cohorts[i].0.is_empty() {
                centroids[i] = geom::Point {
                    x: average_f32(&cohorts[i].0),
                    y: average_f32(&cohorts[i].1),
                    label: None,
                };
            }
        }
    }
}

pub fn cluster(
    points: &mut Vec<geom::Point>,
    k: usize,
    n: usize,
    seed: u64,
) -> Vec<geom::Point> {
    let mut centroids: Vec<geom::Point> =
        centroids(&geom::bounds(points), k, seed);
    for _ in 0..n {
        label_points(points, &centroids);
        adjust_centroids(&points, &mut centroids, k);
    }
    centroids
}
