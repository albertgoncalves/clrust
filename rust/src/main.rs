mod test;

use std::f32;
use std::io::{stdin, Read};

const N: usize = 5; /* # of columns */
const COLUMN_X: usize = 3; /* zero-index of X column */
const COLUMN_Y: usize = 4; /*           ... Y column */

#[derive(Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq)]
struct Bounds {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

fn row_to_point(
    row: &str,
    n: usize,
    column_x: usize,
    column_y: usize,
) -> Option<Point> {
    let items: Vec<&str> = row.split(',').collect::<Vec<&str>>();
    if (items.len() == n)
        && (column_x != column_y)
        && (column_x < n)
        && (column_y < n)
    {
        return items[column_x].parse().ok().and_then(|x| {
            items[column_y]
                .parse()
                .ok()
                .and_then(|y| Some(Point { x, y }))
        });
    }
    None
}

fn bounds(points: &[Point]) -> Bounds {
    let mut bounds = Bounds {
        min_x: f32::MAX,
        max_x: 0.0,
        min_y: f32::MAX,
        max_y: 0.0,
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

fn distance(a: &Point, b: &Point) -> f32 {
    let x: f32 = a.x - b.x;
    let y: f32 = a.y - b.y;
    ((x * x) + (y * y)).sqrt()
}

fn read_stdin() -> Result<String, std::io::Error> {
    let mut buffer: String = String::new();
    stdin().read_to_string(&mut buffer).map(|_| buffer)
}

fn main() {
    if let Ok(buffer) = read_stdin() {
        let lines: Vec<&str> = buffer.split('\n').collect::<Vec<&str>>();
        let mut points: Vec<Point> = Vec::with_capacity(lines.len());
        for line in lines {
            if let Some(point) = row_to_point(line, N, COLUMN_X, COLUMN_Y) {
                points.push(point)
            }
        }
        println!("{:#?}", points);
        if 4 < points.len() {
            println!("{}", distance(&points[1], &points[3]));
        }
        println!("{:?}", bounds(&points));
    }
}
