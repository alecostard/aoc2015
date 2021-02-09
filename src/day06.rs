pub fn day06() -> () {
    let input = std::fs::read_to_string("./inputs/day06.txt").unwrap();
    let instructions: Vec<Instruction> = input.trim().split('\n').map(parse_instruction).collect();

    // first part
    // let mut state1: State1 = [[false; 1000]; 1000];
    // for instruction in instructions {
    //     execute_instruction1(&mut state1, instruction);
    // }
    // println!("{}", count_on(&state1));

    // second part
    let mut state2: State2 = vec![vec![0; 1000]; 1000];
    for instruction in instructions {
        execute_instruction2(&mut state2, instruction);
    }
    println!("{}", total_brightness(&state2));
}

type Position = (usize, usize);

#[derive(Debug)]
enum Instruction {
    Toggle(Position, Position),
    TurnOn(Position, Position),
    TurnOff(Position, Position),
}

fn parse_instruction(line: &str) -> Instruction {
    let words: Vec<String> = line
        .replace("turn ", "turn")
        .split(' ')
        .map(String::from)
        .collect();

    let sw_corner = parse_position(&words[1]);
    let ne_corner = parse_position(&words[3]);

    if words[0].starts_with("toggle") {
        Instruction::Toggle(sw_corner, ne_corner)
    } else if words[0].starts_with("turnon") {
        Instruction::TurnOn(sw_corner, ne_corner)
    } else if words[0].starts_with("turnoff") {
        Instruction::TurnOff(sw_corner, ne_corner)
    } else {
        panic!()
    }
}

fn parse_position(pos: &String) -> Position {
    let mut iterator = pos.split(',');
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let y: usize = iterator.next().unwrap().parse().unwrap();
    (x, y)
}

type State1 = [[bool; 1000]; 1000];

fn execute_instruction1(state: &mut State1, instruction: Instruction) {
    use Instruction::*;
    match instruction {
        Toggle((xl, yl), (xh, yh)) => {
            for i in xl..=xh {
                for j in yl..=yh {
                    state[i][j] = !state[i][j];
                }
            }
        }

        TurnOn((xl, yl), (xh, yh)) => {
            for i in xl..=xh {
                for j in yl..=yh {
                    state[i][j] = true;
                }
            }
        }

        TurnOff((xl, yl), (xh, yh)) => {
            for i in xl..=xh {
                for j in yl..=yh {
                    state[i][j] = false;
                }
            }
        }
    }
}

fn count_on(state: &State1) -> usize {
    let mut total = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if state[i][j] {
                total += 1;
            }
        }
    }
    total
}

type State2 = Vec<Vec<usize>>;

fn execute_instruction2(state: &mut State2, instruction: Instruction) {
    use Instruction::*;
    match instruction {
        Toggle((xl, yl), (xh, yh)) => {
            for i in xl..=xh {
                for j in yl..=yh {
                    state[i][j] += 2;
                }
            }
        }

        TurnOn((xl, yl), (xh, yh)) => {
            for i in xl..=xh {
                for j in yl..=yh {
                    state[i][j] += 1;
                }
            }
        }

        TurnOff((xl, yl), (xh, yh)) => {
            for i in xl..=xh {
                for j in yl..=yh {
                    if state[i][j] > 0 {
                        state[i][j] -= 1;
                    }
                }
            }
        }
    }
}

fn total_brightness(state: &State2) -> usize {
    let mut total = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            total += state[i][j];
        }
    }
    total
}

#[test]
fn examples1() {
    let mut state: State1 = [[false; 1000]; 1000];
    execute_instruction1(&mut state, Instruction::TurnOn((0, 0), (999, 999)));
    assert!(1_000_000 == count_on(&state));

    state = [[false; 1000]; 1000];
    execute_instruction1(&mut state, Instruction::TurnOn((0, 0), (999, 0)));
    assert!(1_000 == count_on(&state));

    state = [[false; 1000]; 1000];
    execute_instruction1(&mut state, Instruction::TurnOn((499, 499), (500, 500)));
    assert!(4 == count_on(&state));
}

#[test]
fn examples2() {
    let mut state: State2 = vec![vec![0; 1000]; 1000];
    execute_instruction2(&mut state, Instruction::TurnOn((0, 0), (0, 0)));
    assert!(1 == total_brightness(&state));

    state = vec![vec![0; 1000]; 1000];
    execute_instruction2(&mut state, Instruction::Toggle((0, 0), (999, 999)));
    assert!(2_000_000 == total_brightness(&state));
}
