fn resolve_problem(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let mut game = line.split(": ");
        game.next().expect("Should work");

        let mut game = game.next().expect("Should work").split(" | ");

        let winning_numbers = game.next().expect("Should work").trim();
        let numbers = game.next().expect("Should work").trim();

        let mut winning_numbers_list = Vec::new();
        let mut numbers_list = Vec::new();
        winning_numbers
            .split(' ')
            .for_each(|n| winning_numbers_list.push(n));
        numbers.split(' ').for_each(|n| numbers_list.push(n));

        winning_numbers_list.sort();
        winning_numbers_list.dedup();
        numbers_list.sort();
        numbers_list.dedup();

        if winning_numbers_list.contains(&"") {
            winning_numbers_list.remove(0);
        }
        if numbers_list.contains(&"") {
            numbers_list.remove(0);
        }

        let mut local_result = 0;
        for number in winning_numbers_list {
            if numbers_list.contains(&number) {
                if local_result == 0 {
                    local_result += 1
                } else {
                    local_result *= 2
                }
            }
        }
        result += local_result;
    }

    result
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
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = resolve_problem(input);
        assert_eq!(13, output);
    }
}
