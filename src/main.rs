use nom::branch::alt;
use std::fs;

use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{digit1, space1};
use nom::combinator::{map_res, opt};
use nom::multi::many1;
use nom::sequence::{delimited, terminated};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Color {
    Blue(i32),
    Red(i32),
    Green(i32),
}

impl Color {
    fn to_inner(self) -> i32 {
        match self {
            Color::Blue(n) => n,
            Color::Green(n) => n,
            Color::Red(n) => n,
        }
    }
}

#[derive(Debug)]
struct GameInfo {
    colors: Vec<Color>,
}

#[derive(Debug)]
struct LineData {
    game_number: i32,
    sets: Vec<GameInfo>,
}

fn get_game_number(input: &str) -> IResult<&str, i32> {
    delimited(
        tag_no_case("game "),
        map_res(digit1, |s: &str| s.parse::<i32>()),
        tag_no_case(":"),
    )(input)
}

// 3 blue
fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        map_res(terminated(digit1, tag_no_case(" blue")), |s: &str| {
            s.parse::<i32>().map(|n| Color::Blue(n))
        }),
        map_res(terminated(digit1, tag_no_case(" green")), |s: &str| {
            s.parse::<i32>().map(|n| Color::Green(n))
        }),
        map_res(terminated(digit1, tag_no_case(" red")), |s: &str| {
            s.parse::<i32>().map(|n| Color::Red(n))
        }),
    ))(input)
}

fn parse_sets(input: &str) -> Vec<GameInfo> {
    let sets = input.split(';');
    sets.map(|set| {
        many1(delimited(space1, parse_color, opt(tag(","))))(set)
            .expect("Failed To parse colors")
            .1
    })
    .map(|colors| GameInfo { colors })
    .collect::<Vec<_>>()
}

fn decode_line(line: &str) -> LineData {
    let (rest, game_number) = get_game_number(line).expect("Failed to parse game number");
    let sets = parse_sets(rest);
    LineData { game_number, sets }
}

#[derive(Debug)]
enum GameResult {
    Possible(i32),
    Impossible(i32),
}

fn calculate_min_required(game: &LineData) -> Vec<Color> {
    let mut min_required_blue = 0;
    let mut min_required_green = 0;
    let mut min_required_red = 0;
    for set in &game.sets {
        for color in &set.colors {
            match color {
                Color::Blue(n) => {
                    if n > &min_required_blue {
                        min_required_blue = *n;
                    }
                }
                Color::Green(n) => {
                    if n > &min_required_green {
                        min_required_green = *n;
                    }
                }
                Color::Red(n) => {
                    if n > &min_required_red {
                        min_required_red = *n;
                    }
                }
            }
        }
    }
    vec![
        Color::Blue(min_required_blue),
        Color::Green(min_required_green),
        Color::Red(min_required_red),
    ]
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut sum_power_of_sets = 0;
    for line in input.lines() {
        println!("{}", line);
        let game = decode_line(line);
        println!("{:?}", game);
        let result = calculate_min_required(&game);
        println!("{:?}", result);
        let power_of_sets = result
            .into_iter()
            .map(|color| color.to_inner())
            .product::<i32>();
        println!("Power of sets: {}", power_of_sets);
        sum_power_of_sets += power_of_sets;
    }
    println!("Sum of power of sets: {}", sum_power_of_sets);
}
