use std::{collections::HashMap};

pub fn day07() {
    let input = std::fs::read_to_string("./inputs/day07.txt").unwrap();
    // part 2
    // let input = input.replace("19138 -> b", "16076 -> b");

    let mut instructions: Vec<&str> = input.lines().collect();

    let mut wires: HashMap<&str, u16> = HashMap::new();

    let mut i = 0;
    while instructions.len() > 0 {
        let instruction = instructions[i];

        let dst = instruction.split(" -> ").nth(1).unwrap().trim();
        let rule = instruction.split(" -> ").nth(0).unwrap().trim();

        if wires.contains_key(dst) {
            instructions.swap_remove(i);
            i = 0;
            continue;
        }

        if rule.split_whitespace().count() == 1 {
            if let Ok(n) = rule.parse::<u16>() {
                wires.insert(dst, n);
            } else if let Some(&n) = wires.get(rule) {
                wires.insert(dst, n);
            }
        } else if rule.split_whitespace().count() == 2 {
            let src = rule.split_whitespace().nth(1).unwrap();
            if let Some(&n) = wires.get(src) {
                wires.insert(dst, !n);
            }
        } else if rule.split_whitespace().count() == 3 {
            if rule.starts_with("1 AND") {
                let y = rule.split_whitespace().nth(2).unwrap();
                if let Some(&n) = wires.get(y) {
                    wires.insert(dst, 1 & n);
                }
            } else {
                let mut terms = rule.split_whitespace();
                let x = terms.next().unwrap().trim();
                let op = terms.next().unwrap().trim();
                let y = terms.next().unwrap().trim();

                if op == "AND" || op == "OR" {
                    if let (Some(&x), Some(&y)) = (wires.get(x), wires.get(y)) {
                        if op == "AND" {
                            wires.insert(dst, x & y);
                        } else {
                            wires.insert(dst, x | y);
                        }
                    }
                } else {
                    if let (Some(&x), Ok(y)) = (wires.get(x), y.parse::<u16>()) {
                        if op == "LSHIFT" {
                            wires.insert(dst, x << y);
                        } else {
                            wires.insert(dst, x >> y);
                        }
                    }
                }
            }
        }

        i = (i + 1) % instructions.len();
    }

    println!("a = {}", wires.get("a").unwrap());
}
