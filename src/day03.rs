use std::collections::HashSet;

pub fn day03() {
    let input = std::fs::read_to_string("./inputs/day03.txt").expect("");
    let directions = parse_directions(input);
    println!("{}", positions_visited(&directions));
    println!("{}", positions_visited_with_robot(&directions));
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn parse_directions(input: String) -> Vec<Direction> {
    input.chars().map(parse_direction).collect()
}

fn parse_direction(c: char) -> Direction {
    use Direction::*;
    match c {
        '^' => North,
        'v' => South,
        '>' => East,
        '<' => West,
        _ => panic!(),
    }
}

type Position = (i32, i32);

fn follow_direction(position: Position, direction: &Direction) -> Position {
    use Direction::*;
    let (x, y) = position;
    match direction {
        North => (x, y + 1),
        South => (x, y - 1),
        East => (x + 1, y),
        West => (x - 1, y),
    }
}

fn positions_visited(directions: &Vec<Direction>) -> i32 {
    let mut current = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(current);
    for direction in directions {
        current = follow_direction(current, direction);
        visited.insert(current);
    }
    visited.len() as i32
}

fn positions_visited_with_robot(directions: &Vec<Direction>) -> i32 {
    let mut santa_current = (0, 0);
    let mut robot_current = (0, 0);
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for (i, direction) in directions.iter().enumerate() {
        if i % 2 == 0 {
            santa_current = follow_direction(santa_current, direction);
            visited.insert(santa_current);
        } else {
            robot_current = follow_direction(robot_current, direction);
            visited.insert(robot_current);
        }
    }
    visited.len() as i32
}

#[test]
fn positions_visited_tests() {
    assert!(2 == positions_visited(&parse_directions(String::from(">"))));
    assert!(4 == positions_visited(&parse_directions(String::from("^>v<"))));
    assert!(2 == positions_visited(&parse_directions(String::from("^v^v^v^v^v"))));
}

#[test]
fn positions_visited_with_robot_tests() {
    assert!(3 == positions_visited_with_robot(&parse_directions(String::from("^v"))));
    assert!(3 == positions_visited_with_robot(&parse_directions(String::from("^>v<"))));
    assert!(11 == positions_visited_with_robot(&parse_directions(String::from("^v^v^v^v^v"))));
}