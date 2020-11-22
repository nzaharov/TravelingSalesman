use rand::prelude::*;
use std::fmt;

const K: usize = 5;

pub type Path = Vec<usize>;
pub type PathLen = usize;

pub struct Map {
    city_count: usize,
    matrix: Vec<Vec<usize>>,
}

impl Map {
    pub fn new(size: usize, rng: &mut ThreadRng) -> Self {
        let mut matrix = vec![vec![0; size]; size];

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if i != j {
                    let weight = rng.gen_range(1, K * size);
                    matrix[i][j] = weight;
                    matrix[j][i] = weight;
                }
            }
        }

        Self {
            city_count: size,
            matrix,
        }
    }

    pub fn get_distance(&self, a: usize, b: usize) -> usize {
        self.matrix[a][b]
    }

    pub fn get_path_length(&self, path: &Path) -> PathLen {
        path.iter()
            .zip(path.iter().skip(1))
            .fold(self.get_distance(0, path[0]), |acc, (&a, &b)| {
                acc + self.get_distance(a, b)
            })
    }

    pub fn get_random_path(&self, rng: &mut ThreadRng) -> Path {
        let mut path: Path = (1..self.city_count).collect();
        path.shuffle(rng);
        path
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = self.matrix.iter().fold(String::new(), |acc, row| {
            let row = row
                .iter()
                .map(|c| format!("{:>3}", c))
                .collect::<Vec<String>>()
                .join(" ");
            acc + &row + "\n\n"
        });
        write!(f, "{}", result)
    }
}
