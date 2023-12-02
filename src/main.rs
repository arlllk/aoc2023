use nom::branch::alt;
use std::fs;

use nom::bytes::complete::{tag, tag_no_case, take};
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_res, opt, value};
use nom::multi::many1;
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};

const RED_CUBES: Color = Color::Red(12);
const GREEN_CUBES: Color = Color::Green(13);
const BLUE_CUBES: Color = Color::Blue(14);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Color {
    Blue(i32),
    Red(i32),
    Green(i32),
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

fn is_possible(game: &LineData) -> GameResult {
    for set in &game.sets {
        for color in &set.colors {
            match color {
                Color::Blue(_) => {
                    if color > &BLUE_CUBES {
                        return GameResult::Impossible(game.game_number);
                    }
                }
                Color::Green(_) => {
                    if color > &GREEN_CUBES {
                        return GameResult::Impossible(game.game_number);
                    }
                }
                Color::Red(_) => {
                    if color > &RED_CUBES {
                        return GameResult::Impossible(game.game_number);
                    }
                }
            }
        }
    }
    GameResult::Possible(game.game_number)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut sum_id = 0;
    for line in input.lines() {
        println!("{}", line);
        let game = decode_line(line);
        println!("{:?}", game);
        let result = is_possible(&game);
        println!("{:?}", result);
        match result {
            GameResult::Possible(id) => sum_id += id,
            GameResult::Impossible(_) => {}
        }
    }
    println!("Sum of possible games: {}", sum_id);
}
