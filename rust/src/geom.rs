use std::f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Bounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

#[allow(dead_code)]
pub fn bounds(points: &[Point]) -> Bounds {
    let mut bounds: Bounds = Bounds {
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

pub fn distance(a: Point, b: Point) -> f32 {
    let x: f32 = a.x - b.x;
    let y: f32 = a.y - b.y;
    ((x * x) + (y * y)).sqrt()
}
