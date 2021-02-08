use std::{cmp::max, fs};

pub fn day02() {
    let input = fs::read_to_string("./inputs/day02.txt").expect("");
    let dimensions: Vec<(i32, i32, i32)> = input
        .trim()
        .split("\n")
        .map(|s| parse_dimensions(s))
        .collect();

    println!("{}", total_paper_area(&dimensions));
    println!("{}", total_ribbon_length(&dimensions));
}

fn parse_dimensions(s: &str) -> (i32, i32, i32) {
    let dimensions: Vec<&str> = s.split("x").collect();
    let l = dimensions[0].parse::<i32>().expect("");
    let w = dimensions[1].parse::<i32>().expect("");
    let h = dimensions[2].parse::<i32>().expect("");
    (l, w, h)
}

fn box_area(dims: &(i32, i32, i32)) -> i32 {
    let (l, w, h) = dims;
    let largest = max(l, max(w, h));
    2 * l * w + 2 * w * h + 2 * l * h + l * w * h / largest
}

fn total_paper_area(dimensions: &Vec<(i32, i32, i32)>) -> i32 {
    dimensions.iter().map(box_area).sum()
}

fn ribbon_length(dims: &(i32, i32, i32)) -> i32 {
    let (l, w, h) = dims;
    let largest = max(l, max(w, h));
    2 * (l + w + h - largest) + l * w * h
}

fn total_ribbon_length(dimensions: &Vec<(i32, i32, i32)>) -> i32 {
    dimensions.iter().map(ribbon_length).sum()
}

#[test]
fn box_area_tests() {
    assert!(58 == box_area(&(2, 3, 4)));
    assert!(43 == box_area(&(1, 1, 10)));
}

#[test]
fn ribbon_length_tests() {
    assert!(34 == ribbon_length(&(2, 3, 4)));
    assert!(14 == ribbon_length(&(1, 1, 10)));
}
