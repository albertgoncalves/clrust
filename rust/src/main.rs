mod geom;
mod kmeans;
mod test;

use std::env;
use std::io::{stdin, Read};
use std::process::exit;

const DELIMITER: char = ',';

struct Args {
    n_columns: usize,
    index_x: usize,
    index_y: usize,
    k: usize,
    n_iter: usize,
    seed: u64,
}

fn row_to_point(
    row: &str,
    n: usize,
    index_x: usize,
    index_y: usize,
) -> Option<geom::Point> {
    let items: Vec<&str> = row.split(DELIMITER).collect::<Vec<&str>>();
    if (items.len() == n)
        && (index_x != index_y)
        && (index_x < n)
        && (index_y < n)
    {
        return items[index_x].parse().ok().and_then(|x| {
            items[index_y]
                .parse()
                .ok()
                .and_then(|y| Some(geom::Point { x, y, label: None }))
        });
    }
    None
}

fn read_stdin() -> Result<String, std::io::Error> {
    let mut buffer: String = String::new();
    stdin().read_to_string(&mut buffer).map(|_| buffer)
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    if args.len() == 7 {
        if let (
            Ok(n_columns),
            Ok(index_x),
            Ok(index_y),
            Ok(k),
            Ok(n_iter),
            Ok(seed),
        ) = (
            args[1].parse::<usize>(),
            args[2].parse::<usize>(),
            args[3].parse::<usize>(),
            args[4].parse::<usize>(),
            args[5].parse::<usize>(),
            args[6].parse::<u64>(),
        ) {
            return Args {
                n_columns,
                index_x,
                index_y,
                k,
                n_iter,
                seed,
            };
        }
    }
    eprintln!(
        "usage: {} <n_columns: int> <index_x: int> <index_y: int> <k: int> \
         <n_iter:int> <seed:int>",
        &args[0]
    );
    exit(1);
}

fn main() {
    if let Ok(buffer) = read_stdin() {
        let args: Args = parse_args();
        let lines: Vec<&str> = buffer.split('\n').collect::<Vec<&str>>();
        let mut points: Vec<geom::Point> = Vec::with_capacity(lines.len());
        for line in lines {
            if let Some(point) =
                row_to_point(line, args.n_columns, args.index_x, args.index_y)
            {
                points.push(point)
            }
        }
        let _centroids: Vec<geom::Point> =
            kmeans::cluster(&mut points, args.k, args.n_iter, args.seed);
        let mut csv: String = String::with_capacity(points.len() * 10);
        csv.push_str("x,y,label");
        for point in points {
            if let Some(label) = point.label {
                csv.push('\n');
                csv.push_str(&point.x.to_string());
                csv.push(',');
                csv.push_str(&point.y.to_string());
                csv.push(',');
                csv.push_str(&label.to_string());
            }
        }
        println!("{}", csv);
    }
}
