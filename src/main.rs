use nom::branch::alt;
use std::fs;

use nom::bytes::complete::{tag_no_case, take};
use nom::character::complete::digit1;
use nom::combinator::{map, value};
use nom::multi::many0;
use nom::{IResult, Parser};

fn numbers(input: &str) -> IResult<&str, Vec<u8>> {
    alt((
        value(vec![0], tag_no_case("oh")),
        value(vec![1], tag_no_case("one")),
        value(vec![2], tag_no_case("two")),
        value(vec![3], tag_no_case("three")),
        value(vec![4], tag_no_case("four")),
        value(vec![5], tag_no_case("five")),
        value(vec![6], tag_no_case("six")),
        value(vec![7], tag_no_case("seven")),
        value(vec![8], tag_no_case("eight")),
        value(vec![9], tag_no_case("nine")),
        map(digit1, |s: &str| {
            s.chars()
                .filter_map(|s| s.to_string().as_str().parse::<u8>().ok())
                .collect()
        }),
    ))(input)
}
fn get_number(input: &str) -> IResult<&str, Vec<u8>> {
    let result = numbers(input);
    match result {
        Ok((rest, part)) => {
            println!("({rest}, {part:?})");
            Ok((rest, part))
        }
        Err(_) => {
            let (rest, _) = take(1_usize)(input)?;
            Ok((rest, vec![]))
        }
    }
}

fn get_numbers(input: &str) -> Vec<u8> {
    let (_, numbers) = many0(get_number)(input).expect("Failed to parse input");
    numbers.iter().flatten().cloned().collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut sum: u32 = 0;
    for line in input.lines() {
        println!("{}", line);
        let input = get_numbers(line);
        let first = input[0];
        let last = input[input.len() - 1];
        let input = first * 10 + last;
        println!("{:?}", input);
        sum += input as u32;
    }
    println!("{sum}");
}
