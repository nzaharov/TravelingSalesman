#[warn(clippy::all)]
mod map;

use map::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::io;

const MAX_GENERATIONS: usize = 30;
const POPULATION_SIZE: usize = 100;

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

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let size = buffer.trim_end().parse::<usize>().expect("Invalid input");

    let mut rng = thread_rng();

    let map = Map::new(size, &mut rng);

    println!("Distances:\n\n{}", map);

    let mut population: Vec<Path> = (0..POPULATION_SIZE)
        .map(|_| map.get_random_path(&mut rng))
        .collect();
    let mut fitness: Vec<f64> = population
        .iter()
        .map(|path| 1_f64 / map.get_path_length(path) as f64)
        .collect();

    for _ in 0..MAX_GENERATIONS {
        let dist = WeightedIndex::new(&fitness).unwrap();
        // let index = dist.sample(&mut rng);

        // let alpha_male = population.get(index).unwrap();
        let index = fitness
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        let alpha_male = population.get(index).unwrap();
        let other_indices: Vec<usize> = dist
            .sample_iter(&mut rng)
            .take(POPULATION_SIZE / 2)
            .collect();
        let crossover_point = rng.gen_range(0, size - 1);

        let children = other_indices
            .iter()
            .map(|&i| crossover(alpha_male, population.get(i).unwrap(), crossover_point))
            .flatten()
            .map(|child| mutate(&child, &mut rng))
            .collect::<Vec<Path>>();
        // let children = population
        //     .iter()
        //     .map(|child| mutate(&child, &mut rng))
        //     .collect();

        population = children;
        fitness = population
            .iter()
            .map(|path| 1_f64 / map.get_path_length(path) as f64)
            .collect();

        // println!("{:?}", fitness);

        let best_index = fitness
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        let best_path = population.get(best_index).unwrap();

        println!(
            "Current best: {:?} {}",
            best_path,
            map.get_path_length(best_path)
        );
    }

    Ok(())
}
