use std::fmt::{Display, Formatter};
use std::fs;

static INCREMENT_VALUE: usize = 2;

struct Emptiness {
    lines: Vec<usize>,
    columns: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Galaxy {
    id: usize,
    x: usize,
    y: usize,
}

impl Display for Galaxy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Galaxy {} at ({}, {})", self.id, self.x, self.y)
    }
}

fn expand(galaxies: &[Galaxy], emptiness: Emptiness) -> Vec<Galaxy> {
    let mut galaxies = galaxies.to_vec();
    for galaxy in galaxies.iter_mut() {
        let upper_empty = emptiness.lines.iter().filter(|l| **l < galaxy.y).count();
        let left_empty = emptiness.columns.iter().filter(|c| **c < galaxy.x).count();
        galaxy.x += left_empty * (INCREMENT_VALUE - 1);
        galaxy.y += upper_empty * (INCREMENT_VALUE - 1);
    }
    galaxies
}

fn distance(galaxy1: &Galaxy, galaxy2: &Galaxy) -> usize {
    let x = if galaxy1.x > galaxy2.x {
        galaxy1.x - galaxy2.x
    } else {
        galaxy2.x - galaxy1.x
    };
    let y = if galaxy1.y > galaxy2.y {
        galaxy1.y - galaxy2.y
    } else {
        galaxy2.y - galaxy1.y
    };
    x + y
}

fn all_distances(galaxies: &[Galaxy]) -> Vec<usize> {
    let mut distances = vec![];
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies.iter().skip(i + 1) {
            // println!("Getting distance between {} and {}", galaxy1.id, galaxy2.id);
            distances.push(distance(galaxy1, galaxy2));
        }
    }
    println!("Distances: {:?}", distances);
    distances
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let (galaxies, emptiness) = parse_file(&input);
    galaxies.iter().for_each(|g| println!("{}", g));
    let galaxies = expand(&galaxies, emptiness);
    galaxies.iter().for_each(|g| println!("{}", g));
    let comparisions = all_distances(&galaxies);
    let sum: usize = comparisions.iter().sum();
    println!("Sum: {}", sum);
}

fn parse_file(input: &str) -> (Vec<Galaxy>, Emptiness) {
    let mut galaxies = vec![];
    let mut empty_lines = vec![];
    let mut empty_columns: Vec<_> = (0..input.lines().next().unwrap().len()).collect();
    let mut id = 1;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let galaxy = Galaxy { id, x, y };
                galaxies.push(galaxy);
                id += 1;
            }
        }
        if line.chars().all(|c| c == '.') {
            empty_lines.push(y);
        }
    }
    empty_columns.retain(|v| {
        if galaxies.iter().any(|g| g.x == *v) {
            false
        } else {
            true
        }
    });
    println!("Empty lines: {:?}", empty_lines);
    println!("Empty columns: {:?}", empty_columns);

    (
        galaxies,
        Emptiness {
            lines: empty_lines,
            columns: empty_columns,
        },
    )
}
