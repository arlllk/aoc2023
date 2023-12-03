use std::fs;
use std::sync::atomic::{AtomicU32, Ordering};

static ID: AtomicU32 = AtomicU32::new(0);

trait Coord {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}

#[derive(Debug, Clone, Copy)]
struct DigitCoords {
    x: usize,
    y: usize,
    value: char,
}

impl Coord for DigitCoords {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

#[derive(Debug, Clone)]
struct NumCoords {
    coords: Vec<DigitCoords>,
    value: u32,
    id: u32,
}

fn get_num_coords(input: &[Vec<char>]) -> Vec<NumCoords> {
    fn push(num_coords: &mut Vec<NumCoords>, digit_coords: &mut Vec<DigitCoords>) {
        num_coords.push(NumCoords {
            coords: digit_coords.clone(),
            value: digit_coords
                .iter()
                .map(|digit| digit.value.to_digit(10).unwrap())
                .fold(0, |acc, digit| acc * 10 + digit),
            id: ID.fetch_add(1, Ordering::Relaxed),
        });
        digit_coords.clear();
    }

    let mut num_coords = Vec::new();
    for (x, row) in input.iter().enumerate() {
        let mut digit_coords = Vec::new();
        for (y, item) in row.iter().enumerate() {
            if item.is_ascii_digit() {
                digit_coords.push(DigitCoords { x, y, value: *item });
                if y == row.len() - 1 {
                    push(&mut num_coords, &mut digit_coords);
                }
            } else if !digit_coords.is_empty() {
                push(&mut num_coords, &mut digit_coords);
            }
        }
    }
    num_coords
}

fn seach_around(input: &[Vec<char>], digit: &DigitCoords) -> bool {
    // (x-1,y-1) (x, y-1) (x+1,y-1)
    // (x-1, y)   (x,y)    (x+1,y)
    // (x-1,y+1) (x, y+1) (x+1,y+1)
    let coords = generate_search_field(digit);
    for (x, y) in coords {
        if let (Some(x), Some(y)) = (x, y) {
            if let Some(char) = input.get(x).and_then(|row| row.get(y)) {
                // No queremos ni numeros ni puntos
                if !char.is_ascii_digit() && char != &'.' {
                    return true;
                }
            }
        }
    }
    false
}

fn generate_search_field<T>(digit: &T) -> [(Option<usize>, Option<usize>); 8]
where
    T: Coord,
{
    [
        (digit.x().checked_sub(1), digit.y().checked_sub(1)),
        (Some(digit.x()), digit.y().checked_sub(1)),
        (digit.x().checked_add(1), digit.y().checked_sub(1)),
        (digit.x().checked_sub(1), Some(digit.y())),
        (digit.x().checked_add(1), Some(digit.y())),
        (digit.x().checked_sub(1), digit.y().checked_add(1)),
        (Some(digit.x()), digit.y().checked_add(1)),
        (digit.x().checked_add(1), digit.y().checked_add(1)),
    ]
}

fn get_valid_digits(input: &[Vec<char>], numbers: Vec<NumCoords>) -> Vec<NumCoords> {
    let mut valid_digits = Vec::new();
    'number: for number in numbers {
        for digit in &number.coords {
            if seach_around(input, digit) {
                valid_digits.push(number);
                continue 'number;
            }
        }
    }
    valid_digits
}

#[derive(Debug, Clone, Copy)]
struct Gears {
    x: usize,
    y: usize,
}

impl Coord for Gears {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

fn get_gears(input: &[Vec<char>]) -> Vec<Gears> {
    let mut gears = Vec::new();
    for (x, row) in input.iter().enumerate() {
        for (y, item) in row.iter().enumerate() {
            if item == &'*' {
                gears.push(Gears { x, y });
            }
        }
    }
    println!("Gears: {:?}", gears);
    gears
}

fn get_ratios(gears: &[Gears], valid_numbers: &[NumCoords]) -> Vec<u32> {
    let mut ratios = Vec::new();
    for gear in gears {
        let search_field = generate_search_field(gear);
        let mut numbers = vec![];
        for (x, y) in search_field {
            if let (Some(x), Some(y)) = (x, y) {
                let res: Vec<_> = valid_numbers
                    .iter()
                    .filter_map(|num| {
                        num.coords
                            .iter()
                            .find(|coord| coord.x == x && coord.y == y)
                            .map(|_| num)
                    })
                    .cloned()
                    .collect();
                if res.len() == 1 {
                    let first = res.first().unwrap();
                    if numbers.iter().any(|n: &NumCoords| n.id == first.id) {
                        continue;
                    }
                    numbers.push(first.clone());
                    continue;
                }
            }
        }

        if numbers.len() == 2 {
            let ratio = numbers[0].value * numbers[1].value;
            ratios.push(ratio);
            println!(
                "Numbers: {:?}",
                numbers.iter().map(|n| n.value).collect::<Vec<_>>()
            );
            println!("Ratio: {:?}", ratio);
        }
    }
    ratios
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let coords = get_num_coords(&matrix);
    //println!("Coords: {:?}", coords);
    println!("Coords len: {}", coords.len());
    let valid_numbers = get_valid_digits(&matrix, coords);
    let gears = get_gears(&matrix);
    let ratios = get_ratios(&gears, &valid_numbers);
    //println!("Valid numbers: {:?}", valid_numbers);
    println!(
        "Sum: {}",
        valid_numbers.iter().map(|g| g.value).sum::<u32>()
    );
    println!("Sum ratios {}", ratios.iter().sum::<u32>());
}
