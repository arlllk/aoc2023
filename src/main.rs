use std::fs;

#[derive(Debug, Clone)]
struct DigitCoords {
    x: usize,
    y: usize,
    value: char,
}

#[derive(Debug, Clone)]
struct NumCoords {
    coords: Vec<DigitCoords>,
    value: u32,
}

fn get_num_coords(input: &[Vec<char>]) -> Vec<NumCoords> {
    fn push(num_coords: &mut Vec<NumCoords>, digit_coords: &mut Vec<DigitCoords>) {
        num_coords.push(NumCoords {
            coords: digit_coords.clone(),
            value: digit_coords
                .iter()
                .map(|digit| digit.value.to_digit(10).unwrap())
                .fold(0, |acc, digit| acc * 10 + digit),
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
    let coords = [
        (digit.x.checked_sub(1), digit.y.checked_sub(1)),
        (Some(digit.x), digit.y.checked_sub(1)),
        (digit.x.checked_add(1), digit.y.checked_sub(1)),
        (digit.x.checked_sub(1), Some(digit.y)),
        (digit.x.checked_add(1), Some(digit.y)),
        (digit.x.checked_sub(1), digit.y.checked_add(1)),
        (Some(digit.x), digit.y.checked_add(1)),
        (digit.x.checked_add(1), digit.y.checked_add(1)),
    ];
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

fn get_valid_digits(input: &[Vec<char>], numbers: Vec<NumCoords>) -> Vec<u32> {
    let mut valid_digits = Vec::new();
    'number: for number in numbers {
        for digit in number.coords {
            if seach_around(input, &digit) {
                valid_digits.push(number.value);
                continue 'number;
            }
        }
    }
    valid_digits
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let coords = get_num_coords(&matrix);
    println!("Coords: {:?}", coords);
    println!("Coords len: {}", coords.len());
    let valid_numbers = get_valid_digits(&matrix, coords);
    println!("Valid numbers: {:?}", valid_numbers);
    println!("Sum: {}", valid_numbers.iter().sum::<u32>());
}
