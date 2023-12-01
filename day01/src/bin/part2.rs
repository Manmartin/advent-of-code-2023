use std::collections::HashMap;

fn resolve_problem(input: &str) -> u32 {
    let numbers = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut result = 0;

    for line in input.lines() {
        let mut min_position = line.len();
        let mut max_position = 0;
        let mut first_digit = 0;
        let mut last_digit = 0;
        for (number, value) in &numbers {
            if let Some(position) = line.find(number) {
                if position <= min_position {
                    min_position = position;
                    first_digit = *value;
                }
            }
        }
        for (number, value) in &numbers {
            if let Some(position) = line.rfind(number) {
                if position >= max_position {
                    max_position = position;
                    last_digit = *value;
                }
            }
        }
        result += first_digit * 10 + last_digit;
    }
    return result;
}

fn main() {
    let input = include_str!("input.txt");

    let output = resolve_problem(input);
    println!("{output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let output = resolve_problem(input);
        assert_eq!(281, output);
    }
}
