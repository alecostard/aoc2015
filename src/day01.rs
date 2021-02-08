use std::fs;

pub fn day01() {
    let input = fs::read_to_string("./inputs/day01.txt").expect("");
    println!("{}", instruction_parser(&input));
    println!("{}", first_basement(&input).unwrap());
}


fn instruction_parser(input: &String) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => panic!(),
    })
}

fn first_basement(input: &String) -> Option<usize> {
    let mut current_floor = 0;
    for (i, c) in input.chars().enumerate() {
        current_floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!(),
        };
        if current_floor < 0 {
            return Some(i + 1)
        }
    }
    None
}

#[test]
fn instruction_parser_tests() {
    assert!(0 == instruction_parser(&String::from("(())")));
    assert!(0 == instruction_parser(&String::from("()()")));
    assert!(3 == instruction_parser(&String::from("(((")));
    assert!(3 == instruction_parser(&String::from("(()(()(")));
    assert!(3 == instruction_parser(&String::from("))(((((")));
    assert!(-1 == instruction_parser(&String::from("())")));
    assert!(-1 == instruction_parser(&String::from("))(")));
    assert!(-3 == instruction_parser(&String::from(")))")));
    assert!(-3 == instruction_parser(&String::from(")())())")));
}

#[test]
fn first_basement_tests() {
    assert!(Some(1) == first_basement(&String::from(")")));
    assert!(Some(5) == first_basement(&String::from("()())")));
}