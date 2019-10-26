use rand::distributions::uniform::{UniformFloat, UniformSampler};
use rand::prelude::{SeedableRng, StdRng};
use std::f32;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub label: usize,
}

#[derive(Debug, PartialEq)]
pub struct Bounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

pub fn bounds(points: &[Point]) -> Bounds {
    let mut bounds = Bounds {
        min_x: f32::MAX,
        max_x: f32::MIN,
        min_y: f32::MAX,
        max_y: f32::MIN,
    };
    for point in points {
        if point.x < bounds.min_x {
            bounds.min_x = point.x
        }
        if bounds.max_x < point.x {
            bounds.max_x = point.x
        }
        if point.y < bounds.min_y {
            bounds.min_y = point.y
        }
        if bounds.max_y < point.y {
            bounds.max_y = point.y
        }
    }
    bounds
}

pub fn distance(a: &Point, b: &Point) -> f32 {
    let x: f32 = a.x - b.x;
    let y: f32 = a.y - b.y;
    ((x * x) + (y * y)).sqrt()
}

pub fn centroids(bounds: &Bounds, k: usize, seed: u64) -> Vec<Point> {
    if 0 < k {
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        let x_uniform = UniformFloat::<f32>::new(bounds.min_x, bounds.max_x);
        let y_uniform = UniformFloat::<f32>::new(bounds.min_y, bounds.max_y);
        let mut centroids: Vec<Point> = Vec::with_capacity(k);
        for _ in 0..k {
            let x: f32 = x_uniform.sample(&mut rng);
            let y: f32 = y_uniform.sample(&mut rng);
            centroids.push(Point { x, y, label: 0 });
        }
        return centroids;
    }
    Vec::new()
}
