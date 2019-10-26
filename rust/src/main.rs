mod geom;
mod test;

use std::io::{stdin, Read};

const N: usize = 5; /* # of columns */
const COLUMN_X: usize = 3; /* zero-index of X column */
const COLUMN_Y: usize = 4; /*           ... Y column */

fn row_to_point(
    row: &str,
    n: usize,
    column_x: usize,
    column_y: usize,
) -> Option<geom::Point> {
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
                .and_then(|y| Some(geom::Point { x, y }))
        });
    }
    None
}

fn read_stdin() -> Result<String, std::io::Error> {
    let mut buffer: String = String::new();
    stdin().read_to_string(&mut buffer).map(|_| buffer)
}

fn main() {
    if let Ok(buffer) = read_stdin() {
        let lines: Vec<&str> = buffer.split('\n').collect::<Vec<&str>>();
        let mut points: Vec<geom::Point> = Vec::with_capacity(lines.len());
        for line in lines {
            if let Some(point) = row_to_point(line, N, COLUMN_X, COLUMN_Y) {
                points.push(point)
            }
        }
        println!("{:#?}", points);
        if 4 < points.len() {
            println!("{}", geom::distance(&points[1], &points[3]));
        }
        println!("{:?}", geom::centroids(&geom::bounds(&points), 5, 0));
    }
}
