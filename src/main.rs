use nom::bytes::complete::{tag, tag_no_case, take_until, take_while};
use nom::combinator::{map, map_res, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, terminated};
use nom::IResult;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
struct Card {
    id: u8,
    amount: i32,
    wining_numbers: Vec<u8>,
    selected_numbers: Vec<u8>,
}

fn parse_number(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(
            opt(tag(" ")),
            take_while(|c: char| c.is_ascii_digit()),
            opt(tag(" ")),
        ),
        |s: &str| s.parse::<u8>(),
    )(input)
}

//  41 48 83 86 17
fn decode_list_numbers(input: &str) -> IResult<&str, Vec<u8>> {
    let (_, values) = many1(parse_number)(input)?;
    Ok(("", values))
}

fn extract_card_id(input: &str) -> IResult<&str, u8> {
    let (rest, card_number) = map_res(
        delimited(
            terminated(tag_no_case("Card "), many0(tag(" "))),
            take_until(":"),
            tag(":"),
        ),
        |s: &str| s.parse::<u8>(),
    )(input)?;
    Ok((rest, card_number))
}
fn extract_wining_values(rest: &str) -> IResult<&str, &str> {
    map(
        delimited(tag(" "), take_until("|"), tag("|")),
        |res: &str| res.trim(),
    )(rest)
}

fn extract_selected_values(rest: &str) -> IResult<&str, &str> {
    Ok(("", rest.trim()))
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn decode_card(input: &str) -> IResult<&str, Card> {
    let (rest, card_number) = extract_card_id(input)?;
    let (rest, wining_values) = extract_wining_values(rest)?;
    let (_, selected_values) = extract_selected_values(rest)?;
    let (_, wining_numbers) = decode_list_numbers(wining_values)?;
    let (_, selected_numbers) = decode_list_numbers(selected_values)?;
    let card = Card {
        id: card_number,
        amount: 1,
        wining_numbers,
        selected_numbers,
    };

    Ok(("", card))
}

fn decode_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (_, card) = decode_card(line).unwrap();
            card
        })
        .collect()
}

fn calculate_card_puntuation_initial(card: &Card) -> u32 {
    card.selected_numbers.iter().fold(0, |acc, number| {
        if card.wining_numbers.contains(number) {
            if acc == 0 {
                acc + 1
            } else {
                acc << 1
            }
        } else {
            acc
        }
    })
}

fn calculate_matches(card: &Card) -> u8 {
    card.selected_numbers.iter().fold(0, |acc, number| {
        if card.wining_numbers.contains(number) {
            acc + 1
        } else {
            acc
        }
    })
}

fn calculate_additional_cards(cards: &Vec<Card>) -> Vec<Card> {
    let mut cloned_cards = cards.clone();
    for card in &cloned_cards.clone() {
        add_matches_to_cards(card, &mut cloned_cards);
    }
    cloned_cards
}

fn add_matches_to_cards(card: &Card, cards: &mut [Card]) {
    let number_of_matches = calculate_matches(card);
    let curr_card = cards
        .iter()
        .find(|c| c.id == card.id)
        .map(|c| c.amount)
        .unwrap_or(1);
    for i in 1_u8..=number_of_matches {
        let new_id = card.id + i;
        if let Some(matched_card) = cards.iter_mut().find(|c| c.id == new_id) {
            matched_card.amount += curr_card;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let cards = decode_cards(&input);
    let puntuation = cards
        .iter()
        .map(calculate_card_puntuation_initial)
        .sum::<u32>();
    println!("puntuation: {}", puntuation);
    let new_cards = calculate_additional_cards(&cards);
    println!("new cards: {:?}", new_cards);
    let total_cards = new_cards.into_iter().fold(0, |acc, card| acc + card.amount);
    println!("total cards: {}", total_cards);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_card_id() {
        let test_input = "Card   1: 69 61 27 58 89 52 81 94 40 51 | 43 40 52 90 37 97 89 80 69 42 51 70 94 58 10 73 21 29 61 63 57 79 81 27 35";
        let expected_output= Ok((" 69 61 27 58 89 52 81 94 40 51 | 43 40 52 90 37 97 89 80 69 42 51 70 94 58 10 73 21 29 61 63 57 79 81 27 35", 1));
        assert_eq!(extract_card_id(test_input), expected_output);

        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected_output = Ok((" 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 1));
        assert_eq!(extract_card_id(test_input), expected_output);

        let test_input = "Card 208:  9 67 74 14 59 41 84 60 73 86 | 87 16 27 86 50  7 30 77 64 76 73 71 99 92 23 82  2  5 55 57 40 47 45 72 21";
        let expected_output = Ok(("  9 67 74 14 59 41 84 60 73 86 | 87 16 27 86 50  7 30 77 64 76 73 71 99 92 23 82  2  5 55 57 40 47 45 72 21", 208));
        assert_eq!(extract_card_id(test_input), expected_output);

        let test_input = "Card 213: 79 84 12 86 58 10 11 24 32 26 | 52 94 65 29 89  7 76 80 31 21 78 37 66 69 13 41 93 73 96 16 92 44 62  3 95";
        let expected_output = Ok((" 79 84 12 86 58 10 11 24 32 26 | 52 94 65 29 89  7 76 80 31 21 78 37 66 69 13 41 93 73 96 16 92 44 62  3 95", 213));
        assert_eq!(extract_card_id(test_input), expected_output);
    }

    #[test]
    fn test_extract_wining_values() {
        let test_input = " 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected_output = Ok((" 83 86  6 31 17  9 48 53", "41 48 83 86 17"));
        assert_eq!(extract_wining_values(test_input), expected_output);

        let test_input = "  9 67 74 14 59 41 84 60 73 86 | 87 16 27 86 50  7 30 77 64 76 73 71 99 92 23 82  2  5 55 57 40 47 45 72 21";
        let expected_output = Ok((
            " 87 16 27 86 50  7 30 77 64 76 73 71 99 92 23 82  2  5 55 57 40 47 45 72 21",
            "9 67 74 14 59 41 84 60 73 86",
        ));
        assert_eq!(extract_wining_values(test_input), expected_output);

        let test_input = " 79 84 12 86 58 10 11 24 32 26 | 52 94 65 29 89  7 76 80 31 21 78 37 66 69 13 41 93 73 96 16 92 44 62  3 95";
        let expected_output = Ok((
            " 52 94 65 29 89  7 76 80 31 21 78 37 66 69 13 41 93 73 96 16 92 44 62  3 95",
            "79 84 12 86 58 10 11 24 32 26",
        ));
        assert_eq!(extract_wining_values(test_input), expected_output);
    }

    #[test]
    fn test_extract_selected_values() {
        let test_input = " 83 86  6 31 17  9 48 53";
        let expected_output = Ok(("", "83 86  6 31 17  9 48 53"));
        assert_eq!(extract_selected_values(test_input), expected_output);

        let test_input =
            " 87 16 27 86 50  7 30 77 64 76 73 71 99 92 23 82  2  5 55 57 40 47 45 72 21";
        let expected_output = Ok((
            "",
            "87 16 27 86 50  7 30 77 64 76 73 71 99 92 23 82  2  5 55 57 40 47 45 72 21",
        ));
        assert_eq!(extract_selected_values(test_input), expected_output);

        let test_input =
            " 52 94 65 29 89  7 76 80 31 21 78 37 66 69 13 41 93 73 96 16 92 44 62  3 95";
        let expected_output = Ok((
            "",
            "52 94 65 29 89  7 76 80 31 21 78 37 66 69 13 41 93 73 96 16 92 44 62  3 95",
        ));
        assert_eq!(extract_selected_values(test_input), expected_output);
    }

    #[test]
    fn test_decode_list_numbers() {
        let test_input = "41 48 83 86 17";
        let expected_output = Ok(("", vec![41, 48, 83, 86, 17]));
        assert_eq!(decode_list_numbers(test_input), expected_output);

        let test_input = "83 86  6 31 17  9 48 53";
        let expected_output = Ok(("", vec![83, 86, 6, 31, 17, 9, 48, 53]));
        assert_eq!(decode_list_numbers(test_input), expected_output);

        let test_input = "9 67 74 14 59 41 84 60 73 86";
        let expected_output = Ok(("", vec![9, 67, 74, 14, 59, 41, 84, 60, 73, 86]));
        assert_eq!(decode_list_numbers(test_input), expected_output);

        let test_input = "79 84 12 86 58 10 11 24 32 26";
        let expected_output = Ok(("", vec![79, 84, 12, 86, 58, 10, 11, 24, 32, 26]));
        assert_eq!(decode_list_numbers(test_input), expected_output);

        let test_input = "83 86  6 31 17  9 48 53";
        let expected_output = Ok(("", vec![83, 86, 6, 31, 17, 9, 48, 53]));
        assert_eq!(decode_list_numbers(test_input), expected_output);

        let test_input =
            "87 16 27 86 50  7 30 77 64 76 73 71 99 92 23 82  2  5 55 57 40 47 45 72 21";
        let expected_output = Ok((
            "",
            vec![
                87, 16, 27, 86, 50, 7, 30, 77, 64, 76, 73, 71, 99, 92, 23, 82, 2, 5, 55, 57, 40,
                47, 45, 72, 21,
            ],
        ));
        assert_eq!(decode_list_numbers(test_input), expected_output);

        let test_input =
            "52 94 65 29 89  7 76 80 31 21 78 37 66 69 13 41 93 73 96 16 92 44 62  3 95";
        let expected_output = Ok((
            "",
            vec![
                52, 94, 65, 29, 89, 7, 76, 80, 31, 21, 78, 37, 66, 69, 13, 41, 93, 73, 96, 16, 92,
                44, 62, 3, 95,
            ],
        ));
        assert_eq!(decode_list_numbers(test_input), expected_output);
    }
    #[test]
    fn test_decode_cards() {
        let test_input = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";

        let expected_output = vec![
            Card {
                id: 1,
                amount: 1,
                wining_numbers: vec![41, 48, 83, 86, 17],
                selected_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                id: 2,
                amount: 1,
                wining_numbers: vec![13, 32, 20, 16, 61],
                selected_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Card {
                id: 3,
                amount: 1,
                wining_numbers: vec![1, 21, 53, 59, 44],
                selected_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
        ];

        assert_eq!(decode_cards(test_input), expected_output);
    }

    #[test]
    fn test_calculate_card_puntuation() {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let card = Card {
            id: 1,
            amount: 1,
            wining_numbers: vec![41, 48, 83, 86, 17],
            selected_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(calculate_card_puntuation_initial(&card), 8);
        // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        let card = Card {
            id: 2,
            amount: 1,
            wining_numbers: vec![13, 32, 20, 16, 61],
            selected_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
        };
        assert_eq!(calculate_card_puntuation_initial(&card), 2);
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        let card = Card {
            id: 3,
            amount: 1,
            wining_numbers: vec![1, 21, 53, 59, 44],
            selected_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
        };
        assert_eq!(calculate_card_puntuation_initial(&card), 2);
        // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        let card = Card {
            id: 4,
            amount: 1,
            wining_numbers: vec![41, 92, 73, 84, 69],
            selected_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };
        assert_eq!(calculate_card_puntuation_initial(&card), 1);
        //Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        let card = Card {
            id: 5,
            amount: 1,
            wining_numbers: vec![87, 83, 26, 28, 32],
            selected_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
        };
        assert_eq!(calculate_card_puntuation_initial(&card), 0);
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        let card = Card {
            id: 6,
            amount: 1,
            wining_numbers: vec![31, 18, 13, 56, 72],
            selected_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
        };
        assert_eq!(calculate_card_puntuation_initial(&card), 0);
    }

    #[test]
    fn test_calculate_matches() {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let card = Card {
            id: 1,
            amount: 1,
            wining_numbers: vec![41, 48, 83, 86, 17],
            selected_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(calculate_matches(&card), 4);
        // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        let card = Card {
            id: 2,
            amount: 1,
            wining_numbers: vec![13, 32, 20, 16, 61],
            selected_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
        };
        assert_eq!(calculate_matches(&card), 2);
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        let card = Card {
            id: 3,
            amount: 1,
            wining_numbers: vec![1, 21, 53, 59, 44],
            selected_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
        };
        assert_eq!(calculate_matches(&card), 2);
        // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        let card = Card {
            id: 4,
            amount: 1,
            wining_numbers: vec![41, 92, 73, 84, 69],
            selected_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };
        assert_eq!(calculate_matches(&card), 1);
        //Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        let card = Card {
            id: 5,
            amount: 1,
            wining_numbers: vec![87, 83, 26, 28, 32],
            selected_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
        };
        assert_eq!(calculate_matches(&card), 0);
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        let card = Card {
            id: 6,
            amount: 1,
            wining_numbers: vec![31, 18, 13, 56, 72],
            selected_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
        };
        assert_eq!(calculate_matches(&card), 0);
    }
}
