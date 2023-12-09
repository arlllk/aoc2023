#![feature(slice_group_by)]

use nom::bytes::complete::{tag, take_while};
use nom::character::complete::line_ending;
use nom::combinator::{map, map_res, opt};
use nom::multi::{many0, many1};
use nom::sequence::terminated;
use nom::IResult;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Play {
    hand: Hands,
    cards: Vec<Cards>,
    bet: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Rank {
    rank: u32,
    play: Play,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Cards {
    J,
    N(u8),
    T,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Hands {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn check_if_upgrade(groups: Vec<&[Cards]>) -> Hands {
    let mut groups = groups.clone();
    // sort new_gruop by len desc
    groups.sort_by_key(|b| std::cmp::Reverse(b.len()));
    match groups.len() {
        1 => Hands::FiveOfAKind,
        2 => {
            let first = groups[0];
            let second = groups[1];
            match (first.len(), second.len()) {
                (4, 1) => {
                    if first.contains(&Cards::J) || second.contains(&Cards::J) {
                        Hands::FiveOfAKind
                    } else {
                        Hands::FourOfAKind
                    }
                }
                (3, 2) => {
                    if first.contains(&Cards::J) || second.contains(&Cards::J) {
                        Hands::FiveOfAKind
                    } else {
                        Hands::FullHouse
                    }
                }
                (_, _) => panic!("Invalid hand {:?}", groups),
            }
        }
        3 => {
            let first = groups[0];
            let second = groups[1];
            let third = groups[2];
            match (first.len(), second.len(), third.len()) {
                (3, 1, 1) => {
                    if first.contains(&Cards::J)
                        || second.contains(&Cards::J)
                        || third.contains(&Cards::J)
                    {
                        Hands::FourOfAKind
                    } else {
                        Hands::ThreeOfAKind
                    }
                }
                (2, 2, 1) => {
                    if first.contains(&Cards::J) || second.contains(&Cards::J) {
                        Hands::FourOfAKind
                    } else if third.contains(&Cards::J) {
                        Hands::FullHouse
                    } else {
                        Hands::TwoPair
                    }
                }
                (_, _, _) => panic!("Invalid hand {:?}", groups),
            }
        }
        4 => {
            let first = groups[0];
            let second = groups[1];
            let third = groups[2];
            let fourth = groups[3];
            match (first.len(), second.len(), third.len(), fourth.len()) {
                (2, 1, 1, 1) => {
                    if first.contains(&Cards::J)
                        || second.contains(&Cards::J)
                        || third.contains(&Cards::J)
                        || fourth.contains(&Cards::J)
                    {
                        Hands::ThreeOfAKind
                    } else {
                        Hands::OnePair
                    }
                }
                (_, _, _, _) => panic!("Invalid hand {:?}", groups),
            }
        }
        5 => {
            let first = groups[0];
            let second = groups[1];
            let third = groups[2];
            let fourth = groups[3];
            let fifth = groups[4];
            match (
                first.len(),
                second.len(),
                third.len(),
                fourth.len(),
                fifth.len(),
            ) {
                (1, 1, 1, 1, 1) => {
                    if first.contains(&Cards::J)
                        || second.contains(&Cards::J)
                        || third.contains(&Cards::J)
                        || fourth.contains(&Cards::J)
                        || fifth.contains(&Cards::J)
                    {
                        Hands::OnePair
                    } else {
                        Hands::HighCard
                    }
                }
                (_, _, _, _, _) => panic!("Invalid hand {:?}", groups),
            }
        }
        _ => panic!("Invalid hand {:?}", groups),
    }
}

fn parse_cards(input: &str) -> Vec<Cards> {
    input
        .chars()
        .map(|c| match c {
            'A' => Cards::A,
            'K' => Cards::K,
            'Q' => Cards::Q,
            'J' => Cards::J,
            'T' => Cards::T,
            '2'..='9' => Cards::N(c.to_digit(10).unwrap() as u8),
            _ => panic!("Invalid card"),
        })
        .collect::<Vec<_>>()
}

fn parse_cards_part(input: &str) -> IResult<&str, Vec<Cards>> {
    terminated(
        map(
            take_while(|c: char| {
                c == 'A'
                    || c == 'K'
                    || c == 'Q'
                    || c == 'J'
                    || c == 'T'
                    || c == '2'
                    || c == '3'
                    || c == '4'
                    || c == '5'
                    || c == '6'
                    || c == '7'
                    || c == '8'
                    || c == '9'
            }),
            |s: &str| parse_cards(s),
        ),
        many0(tag(" ")),
    )(input)
}

fn parse_bet(input: &str) -> IResult<&str, u64> {
    map_res(take_while(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u64>()
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<Cards>, u64)> {
    let (input, cards) = parse_cards_part(input)?;
    let (input, bet) = parse_bet(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, (cards, bet)))
}

fn parse_file(input: &str) -> IResult<&str, Vec<(Vec<Cards>, u64)>> {
    many1(parse_line)(input)
}

fn calculate_play(cards: Vec<Cards>, bet: u64) -> Play {
    let mut ord_cards = cards.clone();
    ord_cards.sort();
    let grouped = ord_cards.group_by(|a, b| a == b).collect::<Vec<_>>();
    let hand = check_if_upgrade(grouped);
    Play { cards, bet, hand }
}

fn asing_rank(plays: Vec<Play>) -> Vec<Rank> {
    let mut rank = 1;
    plays
        .into_iter()
        .map(|play| {
            println!("{} {:?}", rank, play);
            let ranked = Rank { rank, play };
            rank += 1;
            ranked
        })
        .collect::<Vec<_>>()
}

fn calculate_earning(rankeds: Vec<Rank>) -> u64 {
    rankeds
        .iter()
        .map(|rank| rank.play.bet * rank.rank as u64)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let (_, plays) = parse_file(&input).unwrap();
    plays.iter().for_each(|(cards, bet)| {
        println!("{:?} {}", cards, bet);
    });
    let mut plays = plays
        .into_iter()
        .map(|(cards, bet)| calculate_play(cards, bet))
        .collect::<Vec<_>>();
    plays.sort();
    let rankeds = asing_rank(plays);
    rankeds.iter().for_each(|rank| {
        println!("{:?}", rank);
    });
    let earning = calculate_earning(rankeds);
    println!("{}", earning);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_cards() {
        let input = "32T3K";
        let expected = vec![Cards::N(3), Cards::N(2), Cards::T, Cards::N(3), Cards::K];
        let actual = parse_cards(input);
        assert_eq!(expected, actual);

        let input = "6A4K37KK54Q7J45TTT745Q845K6568668";
        let expected = vec![
            Cards::N(6),
            Cards::A,
            Cards::N(4),
            Cards::K,
            Cards::N(3),
            Cards::N(7),
            Cards::K,
            Cards::K,
            Cards::N(5),
            Cards::N(4),
            Cards::Q,
            Cards::N(7),
            Cards::J,
            Cards::N(4),
            Cards::N(5),
            Cards::T,
            Cards::T,
            Cards::T,
            Cards::N(7),
            Cards::N(4),
            Cards::N(5),
            Cards::Q,
            Cards::N(8),
            Cards::N(4),
            Cards::N(5),
            Cards::K,
            Cards::N(6),
            Cards::N(5),
            Cards::N(6),
            Cards::N(8),
            Cards::N(6),
            Cards::N(6),
            Cards::N(8),
        ];
        let actual = parse_cards(input);
        assert_eq!(expected, actual);
    }
}
