use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
enum PointType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

impl PointType {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Unknown char: {}", c),
        }
    }
    fn into_char(self) -> char {
        match self {
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::NorthEast => 'L',
            Self::NorthWest => 'J',
            Self::SouthWest => '7',
            Self::SouthEast => 'F',
            Self::Ground => '.',
            Self::Start => 'S',
        }
    }

    fn is_mirror(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Horizontal, Self::Horizontal) => true,
            (Self::Vertical, Self::Vertical) => true,
            (Self::NorthEast, Self::NorthWest) => true,
            (Self::SouthEast, Self::SouthWest) => true,
            (Self::Start, _) => true,
            (_, _) => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
struct Coords {
    x: i64,
    y: i64,
}

impl Display for Coords {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
struct Position {
    coords: Coords,
    point_type: PointType,
}

impl Position {
    pub fn new(x: i64, y: i64, point_type: PointType) -> Self {
        Self {
            coords: Coords { x, y },
            point_type,
        }
    }

    fn get_north(&self, matrix: &[&[Self]]) -> Option<Self> {
        let y = self.coords.y - 1;
        if y < 0 {
            return None;
        }
        let x = self.coords.x;
        Some(matrix[y as usize][x as usize].clone())
    }

    fn get_south(&self, matrix: &[&[Self]]) -> Option<Self> {
        let y = self.coords.y + 1;
        if y >= matrix.len() as i64 {
            return None;
        }
        let x = self.coords.x;
        Some(matrix[y as usize][x as usize].clone())
    }

    fn get_east(&self, matrix: &[&[Self]]) -> Option<Self> {
        let x = self.coords.x + 1;
        if x >= matrix[0].len() as i64 {
            return None;
        }
        let y = self.coords.y;
        Some(matrix[y as usize][x as usize].clone())
    }

    fn get_west(&self, matrix: &[&[Self]]) -> Option<Self> {
        let x = self.coords.x - 1;
        if x < 0 {
            return None;
        }
        let y = self.coords.y;
        Some(matrix[y as usize][x as usize].clone())
    }

    pub fn is_inside_of(&self, matrix: &[&[Self]], walls: &[Self]) -> bool {
        let x = self.coords.x;
        let y = self.coords.y;
        if walls.iter().any(|item| item.coords == self.coords) {
            return false;
        }
        let previous = &matrix[y as usize][0..x as usize];
        let count_of_walls = walls
            .iter()
            .filter(|a| a.coords.y == y && a.coords.x < x)
            // Y que no apunte al norte
            .filter(|a| {
                a.point_type != PointType::Horizontal
                    && a.point_type != PointType::SouthWest
                    && a.point_type != PointType::SouthEast
            })
            .count();
        if count_of_walls % 2 == 1 {
            println!("Count of walls for {}: {}", self.coords, count_of_walls);
            return true;
        }
        return false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
struct RelativePosition {
    current_position: Position,
    from: Direction,
}

impl RelativePosition {
    pub fn new(current_position: Position, from: Direction) -> Self {
        Self {
            current_position,
            from,
        }
    }
    fn can_be_used(&self) -> bool {
        let from = self.from;
        let point_type = self.current_position.point_type;
        let can_be_used = match (point_type, from) {
            (PointType::Vertical, Direction::North | Direction::South) => true,
            (PointType::Horizontal, Direction::East | Direction::West) => true,
            (PointType::NorthEast, Direction::North | Direction::East) => true,
            (PointType::NorthWest, Direction::North | Direction::West) => true,
            (PointType::SouthWest, Direction::South | Direction::West) => true,
            (PointType::SouthEast, Direction::South | Direction::East) => true,
            (PointType::Start, _) => true,
            (_, _) => false,
        };
        can_be_used
    }

    pub fn get_next_positions(&self, matrix: &[&[Position]]) -> Vec<Self> {
        let current_position = self.current_position;
        let from_direction = self.from;

        let south = current_position
            .get_south(matrix)
            .map(|a| Self::new(a, Direction::North));
        let north = current_position
            .get_north(matrix)
            .map(|a| Self::new(a, Direction::South));
        let east = current_position
            .get_east(matrix)
            .map(|a| Self::new(a, Direction::West));
        let west = current_position
            .get_west(matrix)
            .map(|a| Self::new(a, Direction::East));
        let posible_positions = match (self.current_position.point_type, from_direction) {
            (PointType::Vertical, Direction::North) => vec![south],
            (PointType::Vertical, Direction::South) => vec![north],
            (PointType::Horizontal, Direction::East) => vec![west],
            (PointType::Horizontal, Direction::West) => vec![east],
            (PointType::NorthEast, Direction::North) => vec![east],
            (PointType::NorthEast, Direction::East) => vec![north],
            (PointType::NorthWest, Direction::North) => vec![west],
            (PointType::NorthWest, Direction::West) => vec![north],
            (PointType::SouthWest, Direction::South) => vec![west],
            (PointType::SouthWest, Direction::West) => vec![south],
            (PointType::SouthEast, Direction::South) => vec![east],
            (PointType::SouthEast, Direction::East) => vec![south],
            (PointType::Start, _) => vec![south, north, east, west],
            (PointType::Ground, _) => vec![],
            (_, _) => vec![],
        };
        let next_positions = posible_positions
            .into_iter()
            .filter_map(|a| a)
            .filter(|a| a.can_be_used())
            .collect::<Vec<_>>();
        assert!(next_positions.len() > 0, "No next positions found");
        next_positions
    }

    pub fn get_next_position(&self, matrix: &[&[Position]]) -> Self {
        let next_positions = self.get_next_positions(matrix);
        if self.current_position.point_type != PointType::Start {
            assert!(
                next_positions.len() < 2,
                "Too many next positions found: {}",
                next_positions.len()
            );
        }
        next_positions[0]
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let matrix = parse_file(&input);
    let matrix = matrix.iter().map(|a| a.as_slice()).collect::<Vec<_>>();
    let initial_position = find_initial_position(&matrix);
    let mut walls = vec![];
    let mut current_position = initial_position;
    walls.push(current_position.current_position);
    println!("Initial position: {:?}", current_position);
    let mut i = 0;
    loop {
        let next_position = current_position.get_next_position(&matrix);
        println!("Current position: {:?}", next_position);
        current_position = next_position;
        i += 1;
        walls.push(current_position.current_position);
        if current_position.current_position.point_type == PointType::Start {
            break;
        }
    }
    println!("Steps: {}", i);
    println!("farthest: {}", i / 2);
    let points = matrix.iter().flat_map(|a| a.iter()).collect::<Vec<_>>();
    let val: Vec<_> = points
        .iter()
        .filter(|a| a.is_inside_of(&matrix, &walls))
        .collect();
    println!("Points inside: {}", val.len());
}

fn parse_file(input: &str) -> Vec<Vec<Position>> {
    let mut matrix = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut positions = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let point_type = PointType::from_char(c);
            let position = Position::new(x as i64, y as i64, point_type);
            positions.push(position);
        }
        matrix.push(positions);
    }
    matrix
}

fn find_initial_position(matrix: &[&[Position]]) -> RelativePosition {
    let start_position = matrix
        .iter()
        .flat_map(|a| a.iter())
        .find(|a| a.point_type == PointType::Start)
        .unwrap();
    RelativePosition::new(*start_position, Direction::None)
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_cards() {}
}
