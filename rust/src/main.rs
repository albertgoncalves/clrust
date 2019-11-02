mod geom;
mod kmeans;
mod math;
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
    threshold: f32,
    loops: usize,
    seed: u64,
}

fn row_to_point(
    row: &str,
    n: usize,
    index_x: usize,
    index_y: usize,
) -> Option<(f32, f32)> {
    let items: Vec<&str> = row.split(DELIMITER).collect::<Vec<&str>>();
    if (items.len() == n)
        && (index_x != index_y)
        && (index_x < n)
        && (index_y < n)
    {
        return items[index_x].parse().ok().and_then(|x| {
            items[index_y].parse().ok().and_then(|y| Some((x, y)))
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
    if args.len() == 8 {
        if let (
            Ok(n_columns),
            Ok(index_x),
            Ok(index_y),
            Ok(k),
            Ok(threshold),
            Ok(loops),
            Ok(seed),
        ) = (
            args[1].parse::<usize>(),
            args[2].parse::<usize>(),
            args[3].parse::<usize>(),
            args[4].parse::<usize>(),
            args[5].parse::<f32>(),
            args[6].parse::<usize>(),
            args[7].parse::<u64>(),
        ) {
            return Args {
                n_columns,
                index_x,
                index_y,
                k,
                threshold,
                loops,
                seed,
            };
        }
    }
    eprintln!(
        "usage: {} <n_columns: int> <index_x: int> <index_y: int> <k: int> \
         <threshold: float> <loops: int> <seed: int>",
        &args[0]
    );
    exit(1);
}

fn write_csv(xs: &[f32], ys: &[f32], labels: &[usize], n: usize) {
    let mut csv: String = String::with_capacity(n * 10);
    csv.push('x');
    csv.push(DELIMITER);
    csv.push('y');
    csv.push(DELIMITER);
    csv.push_str("label");
    for i in 0..n {
        csv.push('\n');
        csv.push_str(&xs[i].to_string());
        csv.push(DELIMITER);
        csv.push_str(&ys[i].to_string());
        csv.push(DELIMITER);
        csv.push_str(&labels[i].to_string());
    }
    println!("{}", csv);
}

fn main() {
    if let Ok(buffer) = read_stdin() {
        let args: Args = parse_args();
        let lines: Vec<&str> = buffer.split('\n').collect::<Vec<&str>>();
        let n: usize = lines.len();
        let mut xs: Vec<f32> = Vec::with_capacity(n);
        let mut ys: Vec<f32> = Vec::with_capacity(n);
        for line in lines {
            if let Some((x, y)) =
                row_to_point(line, args.n_columns, args.index_x, args.index_y)
            {
                xs.push(x);
                ys.push(y);
            }
        }
        if let (Some(()), Some(())) =
            (math::unit_scale_f32(&mut xs), math::unit_scale_f32(&mut ys))
        {
            if let Some((labels, m, iterations, error)) = kmeans::cluster(
                &xs,
                &ys,
                args.k,
                args.threshold,
                args.loops,
                args.seed,
            ) {
                eprintln!(
                    "iterations : {}\n\
                     n          : {}\n\
                     error      : {}",
                    iterations, //
                    m,          //
                    error,
                );
                write_csv(&xs, &ys, &labels, m);
            }
        }
    }
}
