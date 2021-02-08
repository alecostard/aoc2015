use fancy_regex::Regex;

pub fn day05() {
    let input = std::fs::read_to_string("./inputs/day05.txt").unwrap();
    // println!("{}", input.split('\n').filter(|x| is_nice1(x)).count());
    println!("{}", input.split('\n').filter(|x| is_nice2(x)).count());
}

fn is_nice1(word: &str) -> bool {
    let three_vowels = Regex::new(r"^(.*[aeiou].*){3}$").unwrap();
    let repeated = Regex::new(r"(\w)\1").unwrap();
    let forbidden = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    !forbidden.is_match(word).unwrap()
        && three_vowels.is_match(word).unwrap()
        && repeated.is_match(word).unwrap()
}

fn is_nice2(word: &str) -> bool {
    let pair_twice = Regex::new(r"(\w\w).*\1").unwrap();
    let letter_between = Regex::new(r"(\w).\1").unwrap();

    pair_twice.is_match(word).unwrap() && letter_between.is_match(word).unwrap()
}

#[test]
fn examples1() {
    assert!(is_nice1("ugknbfddgicrmopn"));
    assert!(is_nice1("aaa"));
    assert!(!is_nice1("jchzalrnumimnmhp"));
    assert!(!is_nice1("haegwjzuvuyypxyu"));
    assert!(!is_nice1("dvszwmarrgswjxmb"));
}

#[test]
fn examples2() {
    assert!(is_nice2("qjhvhtzxzqqjkmpb"));
    assert!(is_nice2("xxyxx"));
    assert!(!is_nice2("uurcxstgmygtbstg"));
    assert!(!is_nice2("ieodomkazucvgmuy"));
}

