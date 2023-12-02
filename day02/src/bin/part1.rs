macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn get_index(line: &str) -> (u32, String) {
    let mut line = line.split(':');

    let game_index = line.next().unwrap();
    let remainder = line.next().unwrap();
    let output = scan!(game_index, ' ', String, u32);

    let index = output.1.unwrap();
    (index, remainder.to_owned())
}

fn check_grab(grab: &str) -> bool {
    let colors = grab.split(",");

    for color in colors {
        let color = color.trim();
        let output = scan!(color, ' ', u32, String);
        let number = output.0.unwrap();
        let color = output.1.unwrap();

        match color.as_str() {
            "red" => {
                if number > 12 {
                    return false;
                }
            }
            "green" => {
                if number > 13 {
                    return false;
                }
            }
            "blue" => {
                if number > 14 {
                    return false;
                }
            }
            _ => {
                eprintln!("Error: Unknown color");
                std::process::exit(1);
            }
        }
    }

    return true;
}

fn resolve_problem(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let (mut index, remainder) = get_index(line);

        let grabs = remainder.split(";");
        for grab in grabs {
            if !check_grab(grab) {
                index = 0;
                break;
            }
        }
        result += index;
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let output = resolve_problem(input);
        assert_eq!(8, output);
    }
}
