fn resolve_problem(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        for character in line.chars() {
            if character.is_digit(10) {
                first_digit = character.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for character in line.chars().rev() {
            if character.is_digit(10) {
                last_digit = character.to_digit(10).unwrap();
                break;
            }
        }
        result += first_digit + last_digit;
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
        let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let output = resolve_problem(input);
        assert_eq!(142, output);
    }
}
