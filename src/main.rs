#[warn(clippy::all)]
mod cli;
mod map;

use cli::Cli;
use map::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::io;
use structopt::StructOpt;

fn one_point_crossover(p1: &Path, p2: &Path, i: usize) -> Path {
    let (genes1, genes2) = p1.split_at(i);
    genes1
        .iter()
        .chain(p2.iter().filter(|g| genes2.contains(g)))
        .cloned()
        .collect()
}

fn crossover(p1: &Path, p2: &Path, i: usize) -> Vec<Path> {
    let child_p1 = one_point_crossover(p1, p2, i);
    let child_p2 = one_point_crossover(p2, p1, i);

    vec![child_p1, child_p2]
}

fn mutate(path: &Path, rng: &mut ThreadRng) -> Path {
    let a = rng.gen_range(0, path.len());
    let b = rng.gen_range(0, path.len());
    let mut mutated = path.clone();
    mutated.swap(a, b);
    mutated
}

fn get_best_path(population: &Vec<Path>, fitness: &Vec<f64>) -> Path {
    let best_index = fitness
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0;

    population.get(best_index).unwrap().clone()
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let Cli {
        cities,
        generations,
        population: population_size,
    } = args;

    let mut rng = thread_rng();

    let map = Map::new(cities, &mut rng);

    let mut population: Vec<Path> = (0..population_size)
        .map(|_| map.get_random_path(&mut rng))
        .collect();
    let mut fitness: Vec<f64> = population
        .iter()
        .map(|path| 1_f64 / map.get_path_length(path) as f64)
        .collect();

    for gen in 0..generations {
        let alpha_index = fitness
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        let alpha_male = population.get(alpha_index).unwrap();

        let dist = WeightedIndex::new(&fitness).unwrap();
        let other_indices: Vec<usize> = dist
            .sample_iter(&mut rng)
            .take(population_size / 2)
            .collect();

        let crossover_point = rng.gen_range(0, cities - 1);

        let children = other_indices
            .iter()
            .map(|&i| crossover(alpha_male, population.get(i).unwrap(), crossover_point))
            .flatten()
            .map(|child| mutate(&child, &mut rng))
            .collect::<Vec<Path>>();

        population = children;
        fitness = population
            .iter()
            .map(|path| 1_f64 / map.get_path_length(path) as f64)
            .collect();

        if gen == 10 || gen == 30 || gen == 100 || gen == 500 {
            let current_best_path = get_best_path(&population, &fitness);
            println!(
                "{}th generation best: {}",
                gen,
                map.get_path_length(&current_best_path)
            );
        }
    }

    let best_path = get_best_path(&population, &fitness);
    println!("Final best: {}", map.get_path_length(&best_path));

    Ok(())
}
