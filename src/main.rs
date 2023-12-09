use nom::bytes::complete::{tag, take_while};
use nom::character::complete::line_ending;
use nom::combinator::{map_res, opt};
use nom::multi::{many0, many1};
use nom::sequence::delimited;
use nom::IResult;
use std::fs;

#[derive(Debug, Clone)]
struct Pattern {
    values: Vec<i64>,
    child: Option<Box<Pattern>>,
}

fn calculate_progression(pattern: Box<Pattern>) -> Box<Pattern> {
    if pattern.values.iter().all(|x| *x == 0) {
        return pattern;
    }
    let mut pattern = pattern.clone();

    let mut progression = vec![];
    let mut last_value = pattern.values[0];
    for value in pattern.values.iter().skip(1) {
        progression.push(value - last_value);
        last_value = *value;
    }
    let child = Box::new(Pattern {
        values: progression,
        child: None,
    });

    let child = calculate_progression(child.clone());

    pattern.child = Some(child);
    pattern
}

fn calculate_next_value(pattern: Box<Pattern>) -> Option<i64> {
    if pattern.values.iter().all(|x| *x == 0) {
        Some(0)
    } else {
        let child = pattern.child.clone().unwrap();
        let next_value = calculate_next_value(child.clone());
        if let Some(next_value) = next_value {
            return if next_value == 0 {
                pattern.values.last().cloned()
            } else {
                pattern.values.last().map(|v| v + next_value)
            };
        } else {
            panic!(
                "NO se pudo calcular el siguiente valor de {:?}",
                pattern.clone()
            );
        }
    }
}

fn calculate_previous_value(pattern: Box<Pattern>) -> Option<i64> {
    if pattern.values.iter().all(|x| *x == 0) {
        Some(0)
    } else {
        let child = pattern.child.clone().unwrap();
        let next_value = calculate_previous_value(child.clone());
        if let Some(next_value) = next_value {
            return if next_value == 0 {
                pattern.values.first().cloned()
            } else {
                pattern.values.first().map(|v| v - next_value)
            };
        } else {
            panic!(
                "NO se pudo calcular el siguiente valor de {:?}",
                pattern.clone()
            );
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut next = vec![];
    let mut prev = vec![];

    let (_, pattern) = parse_file(&input).unwrap();
    {
        println!("Pattern: {:?}", pattern);
        for pattern in pattern.iter() {
            let x = Box::new(pattern.clone());
            let r = calculate_progression(x).clone();
            let n = calculate_next_value(r.clone());
            next.push(n);
            let p = calculate_previous_value(r);
            prev.push(p);
        }
    }
    println!("Nexts: {:?}", next);
    let sum = next.iter().filter_map(|a| *a).sum::<i64>();
    println!("Sum: {:?}", sum);
    println!("Prev: {:?}", prev);
    let sum_prev = prev.iter().filter_map(|a| *a).sum::<i64>();
    println!("Sum Prev: {:?}", sum_prev);
}

fn parse_file(input: &str) -> IResult<&str, Vec<Pattern>> {
    let (input, values) = many1(parse_node)(input)?;
    Ok((input, values))
}

fn parse_number(input: &str) -> IResult<&str, i64> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }
    map_res(
        take_while(|c: char| c.is_ascii_digit() || c == '-'),
        |s: &str| s.parse::<i64>(),
    )(input)
}

fn parse_node(input: &str) -> IResult<&str, Pattern> {
    let (input, values) = many1(delimited(many0(tag(" ")), parse_number, many0(tag(" "))))(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((
        input,
        Pattern {
            values,
            child: None,
        },
    ))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_cards() {}
}
