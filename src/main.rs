mod macros;
mod traits;

use crate::traits::NuType;
use nom::bytes::complete::{tag, tag_no_case, take_until, take_while};
use nom::character::complete::line_ending;
use nom::combinator::{map, map_res, opt};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated};
use nom::IResult;
use rayon::prelude::*;
use std::fmt::Display;
use std::fs;
use std::slice::Iter;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Time(u64);
impl_nu_type!(Time);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Distance(u64);
impl_nu_type!(Distance);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Speed(u64);
impl_nu_type!(Speed);

impl std::ops::Mul<Speed> for Time {
    type Output = Distance;

    fn mul(self, rhs: Speed) -> Self::Output {
        Distance::new(self.into_inner() * rhs.into_inner())
    }
}

fn parse_list_numbers(input: &str) -> IResult<&str, u64> {
    terminated(
        map_res(take_while(|c: char| c.is_ascii_digit()), |s: &str| {
            s.parse::<u64>()
        }),
        many0(tag(" ")),
    )(input)
}

fn parse_time(input: &str) -> IResult<&str, Vec<Time>> {
    map(
        preceded(
            preceded(tag_no_case("time:"), many0(tag(" "))),
            many1(parse_list_numbers),
        ),
        |s| s.iter().map(|s| Time::new(*s)).collect::<Vec<_>>(),
    )(input)
}

fn parse_distance(input: &str) -> IResult<&str, Vec<Distance>> {
    map(
        preceded(
            preceded(tag_no_case("distance:"), many0(tag(" "))),
            many1(parse_list_numbers),
        ),
        |s| s.iter().map(|s| Distance::new(*s)).collect::<Vec<_>>(),
    )(input)
}

fn join_values<T: NuType + Copy>(values: &[T]) -> u64 {
    values
        .iter()
        .map(|v| v.into_inner().to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}

fn parse_input(input: &str) -> IResult<&str, (Time, Distance)> {
    let (input, times) = parse_time(input)?;
    let (input, _) = line_ending(input)?;
    let (input, distances) = parse_distance(input)?;
    let time_value = Time::new(join_values(&times));
    let distance_value = Distance::new(join_values(&distances));
    Ok((input, (time_value, distance_value)))
}

fn find_distance_archieved(total_time: &Time, time_to_press: Time) -> Distance {
    let remaining_time = total_time - time_to_press;
    let speed = Speed::new(time_to_press.into_inner());
    let distance_archived = remaining_time * speed;
    distance_archived
}

fn find_times_to_press(total_time: &Time, distance_to_beat: &Distance) -> Vec<Time> {
    let mut over = false;
    let mut valid_times = vec![];
    for i in 0..total_time.into_inner() {
        let distance_archived = find_distance_archieved(total_time, Time::new(i));
        if distance_archived > *distance_to_beat {
            valid_times.push(Time::new(i));
            if !over {
                over = true;
            }
        } else if over {
            break;
        }
    }
    valid_times
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let (_, (t, d)) = parse_input(&input).unwrap();
    let (t, d, times) = (t, d, find_times_to_press(&t, &d));
    //println!("Time: {}, Distance: {}, Times: {:?}", t, d, times);
    let count = times.len();
    println!("Total: {}", count);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_map() {}
}
