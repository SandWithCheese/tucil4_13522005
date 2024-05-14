use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type AdjMatrix = Vec<Vec<f64>>;

fn read_input_file(file_path: &str) -> io::Result<(usize, AdjMatrix)> {
    let file = File::open(file_path)?;
    let mut lines = io::BufReader::new(file).lines();
    let n = lines.next().unwrap()?.parse::<usize>().unwrap();
    let mut adj_matrix = vec![vec![f64::INFINITY; n]; n];

    for i in 0..n {
        if let Some(line) = lines.next() {
            let line = line?;
            let values = line
                .split_whitespace()
                .map(|x| {
                    if x == "inf" {
                        f64::INFINITY
                    } else {
                        x.parse().unwrap()
                    }
                })
                .collect::<Vec<f64>>();
            adj_matrix[i] = values;
        }
    }

    Ok((n, adj_matrix))
}

fn tsp(
    i: usize,
    s: &BTreeSet<usize>,
    adj_matrix: &AdjMatrix,
    memo: &mut HashMap<(usize, BTreeSet<usize>), f64>,
) -> f64 {
    if s.is_empty() {
        return adj_matrix[i][0];
    }

    if let Some(&cost) = memo.get(&(i, s.clone())) {
        return cost;
    }

    let mut min_cost = f64::INFINITY;

    for &j in s {
        if adj_matrix[i][j] != f64::INFINITY {
            let mut s_ = s.clone();
            s_.remove(&j);
            let cost = adj_matrix[i][j] + tsp(j, &s_, adj_matrix, memo);
            min_cost = min_cost.min(cost);
        }
    }

    memo.insert((i, s.clone()), min_cost);
    min_cost
}

fn get_path(
    i: usize,
    s: &BTreeSet<usize>,
    adj_matrix: &AdjMatrix,
    memo: &mut HashMap<(usize, BTreeSet<usize>), f64>,
) -> Vec<usize> {
    if s.is_empty() {
        return vec![0];
    }

    let mut min_cost = f64::INFINITY;
    let mut min_path = vec![];

    for &j in s {
        if adj_matrix[i][j] != f64::INFINITY {
            let mut s_ = s.clone();
            s_.remove(&j);
            let cost = adj_matrix[i][j] + tsp(j, &s_, adj_matrix, memo);
            if cost < min_cost {
                min_cost = cost;
                min_path = vec![j];
                min_path.extend(get_path(j, &s_, adj_matrix, memo));
            }
        }
    }

    min_path
}

fn get_tour(
    n: usize,
    adj_matrix: &AdjMatrix,
    memo: &mut HashMap<(usize, BTreeSet<usize>), f64>,
) -> Vec<usize> {
    let path = get_path(0, &(1..n).collect::<BTreeSet<_>>(), adj_matrix, memo);
    let mut tour = path.clone();
    tour.insert(0, 0);
    tour
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_config_file>", args[0]);
        return;
    }

    let input_file = &args[1];
    let (n, adj_matrix) = read_input_file(input_file).expect("Failed to read input file");

    let mut memo = HashMap::new();
    let min_cost = tsp(0, &(1..n).collect::<BTreeSet<_>>(), &adj_matrix, &mut memo);

    let tour = get_tour(n, &adj_matrix, &mut memo);
    let tour: Vec<String> = tour.iter().map(|&x| (x + 1).to_string()).collect();

    println!(
        "Tur yang optimal adalah {} dengan bobot = {}",
        tour.join(", "),
        min_cost
    );
}
