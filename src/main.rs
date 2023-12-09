#![feature(slice_group_by)]

use nom::bytes::complete::{tag, take_while};
use nom::character::complete::line_ending;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};
use rayon::prelude::*;
use std::cmp::{max, min};
use std::fs;
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ProtoNode<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> ProtoNode<'a> {
    #[inline]
    fn new(name: &'a str, left: &'a str, right: &'a str) -> ProtoNode<'a> {
        ProtoNode { name, left, right }
    }

    fn follow(&self, nodes: &'a [ProtoNode], instructions: Instruction) -> &'a ProtoNode {
        match instructions {
            Instruction::Left => self.go_left(nodes),
            Instruction::Right => self.go_right(nodes),
        }
    }

    #[inline]
    fn go_left(&self, nodes: &'a [ProtoNode]) -> &'a ProtoNode {
        nodes
            .iter()
            .find(|n| n.name == self.left)
            .unwrap_or_else(|| panic!("No se encontro el nodo left {} de {}", self.left, self.name))
    }

    #[inline]
    fn go_right(&self, nodes: &'a [ProtoNode]) -> &'a ProtoNode {
        nodes
            .iter()
            .find(|n| n.name == self.right)
            .unwrap_or_else(|| {
                panic!(
                    "No se encontro el nodo right {} de {}",
                    self.right, self.name
                )
            })
    }
}
fn gcd(a: u64, b: u64) -> u64 {
    // The numbers, and also the last bytes
    match ((a, b), (a & 1, b & 1)) {
        // If is equal, is the number
        ((x, y), _) if x == y => y,
        // If one is zero, is the other
        ((0, x), _) | ((x, 0), _) => x,
        // If one is even, divide by two (bitshift) the even number
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        // Is both are even, divide by two (bitshift), and multiply by two the result (bitshift)
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        // If both are odd
        ((x, y), (1, 1)) => {
            // We get the biggest number and the smallest
            let (x, y) = (min(x, y), max(x, y));
            // We substract the smallest to the biggest, and divide by two (bitshift) and call with the smallest
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn escape(instruction: &[Instruction], nodes: &[ProtoNode]) {
    let first = nodes
        .iter()
        .find(|n| n.name == "AAA")
        .unwrap_or_else(|| panic!("No se encontro el nodo AAA"));
    let mut steps = 0;
    let mut current = first;
    loop {
        for i in instruction {
            current = current.follow(nodes, *i);
            steps += 1;
            if current.name == "ZZZ" {
                println!("Escaped in {} steps", steps);
                return;
            }
        }
        println!("current: {:?}", current);
    }
}

fn calculate_scape_values<'a>(
    intructions: &[Instruction],
    nodes: &'a [ProtoNode<'_>],
    start_node: &'a ProtoNode<'_>,
) -> u64 {
    let mut steps = 0_u64;
    let mut current = start_node;
    loop {
        for i in intructions {
            current = current.follow(nodes, *i);
            steps += 1;
            if current.name.ends_with('Z') {
                //println!("Escaped in {} steps", steps);
                return steps;
            }
        }
    }
}

fn simul_scape(instruction: &[Instruction], nodes: &[ProtoNode]) {
    let firsts: Vec<_> = nodes.iter().filter(|n| n.name.ends_with('A')).collect();
    let currents: Vec<_> = firsts;
    let scape_values = currents
        .into_par_iter()
        .map(|c| calculate_scape_values(instruction, nodes, &c))
        .collect::<Vec<_>>();
    let acc = scape_values.iter().fold(1, |acc, val| lcm(acc, *val));
    println!("LCM: {}", acc);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let (_, (instructions, nodes)) = parse_file(&input).unwrap();
    println!("Instructions: {:?}", instructions);
    println!("Nodes: {:?}", nodes);
    //escape(&instructions, &nodes);
    simul_scape(&instructions, &nodes);
}

fn parse_file(input: &str) -> IResult<&str, (Vec<Instruction>, Vec<ProtoNode>)> {
    let (input, instructions) = parse_instructions(input)?;
    let (input, _) = many0(line_ending)(input)?;
    let (input, nodes) = many1(parse_node)(input)?;
    Ok((input, (instructions, nodes)))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    map(
        terminated(many1(tag("L").or(tag("R"))), opt(line_ending)),
        |s: Vec<&str>| {
            s.iter()
                .map(|c| match *c {
                    "L" => Instruction::Left,
                    "R" => Instruction::Right,
                    _ => panic!("No es una instruccion valida {}", c.escape_debug()),
                })
                .collect()
        },
    )(input)
}

fn parse_node(input: &str) -> IResult<&str, ProtoNode> {
    let (input, name) = parse_name(input)?;
    let (input, _) = preceded(
        delimited(many0(tag(" ")), tag("="), many0(tag(" "))),
        tag("("),
    )(input)?;
    let (input, left) = parse_name(input)?;
    let (input, _) = terminated(tag(","), many0(tag(" ")))(input)?;
    let (input, right) = parse_name(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, ProtoNode::new(name, left, right)))
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric())(input)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_cards() {}
}
