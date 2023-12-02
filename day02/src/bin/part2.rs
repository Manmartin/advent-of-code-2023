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

fn get_num_balls(grab: &str) -> (u32, u32, u32) {
    let colors = grab.split(",");

    let (mut red, mut green, mut blue) = (0, 0, 0);
    for color in colors {
        let color = color.trim();
        let output = scan!(color, ' ', u32, String);
        let number = output.0.unwrap();
        let color = output.1.unwrap();

        match color.as_str() {
            "red" => {
                red = number;
            }
            "green" => {
                green = number;
            }
            "blue" => {
                blue = number;
            }
            _ => {
                eprintln!("Error: Unknown color");
                std::process::exit(1);
            }
        }
    }
    return (red, green, blue);
}

fn resolve_problem(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let (_, remainder) = get_index(line);

        let (mut red, mut green, mut blue) = (0, 0, 0);
        let grabs = remainder.split(";");
        for grab in grabs {
            let colors_number = get_num_balls(grab);
            if colors_number.0 > red {
                red = colors_number.0;
            }
            if colors_number.1 > green {
                green = colors_number.1;
            }
            if colors_number.2 > blue {
                blue = colors_number.2;
            }
        }
        result += red * green * blue;
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
        assert_eq!(2286, output);
    }
}
