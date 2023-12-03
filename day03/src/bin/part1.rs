struct Number {
    value: u32,
    start: u32,
    end: u32,
}

fn get_numbers(row: &[u8]) -> Vec<Number> {
    let mut numbers = Vec::new();

    let mut start = 0;
    while start < row.len() {
        if row[start].is_ascii_digit() {
            let mut end = start + 1;
            while end < row.len() && row[end].is_ascii_digit() {
                end += 1;
            }
            let value = std::str::from_utf8(&row[start..end]).unwrap();
            let value = value.parse().unwrap();

            numbers.push(Number {
                value,
                start: start as u32,
                end: end as u32,
            });
            start = end;
        } else {
            start += 1;
        }
    }
    return numbers;
}

fn check_symbol(map: &Vec<&[u8]>, i: usize, j: usize) -> bool {
    if i != 0 && j != 0 && !map[i - 1][j - 1].is_ascii_digit() && map[i - 1][j - 1] != '.' as u8 {
        return true;
    }
    if i != 0 && !map[i - 1][j].is_ascii_digit() && map[i - 1][j] != '.' as u8 {
        return true;
    }
    if i != 0
        && j != map[i].len() - 1
        && !map[i - 1][j + 1].is_ascii_digit()
        && map[i - 1][j + 1] != '.' as u8
    {
        return true;
    }

    if j != 0 && !map[i][j - 1].is_ascii_digit() && map[i][j - 1] != '.' as u8 {
        return true;
    }
    if j != map.len() - 1 && !map[i][j + 1].is_ascii_digit() && map[i][j + 1] != '.' as u8 {
        return true;
    }

    if i != map.len() - 1
        && j != 0
        && !map[i + 1][j - 1].is_ascii_digit()
        && map[i + 1][j - 1] != '.' as u8
    {
        return true;
    }
    if i != map.len() - 1 && !map[i + 1][j].is_ascii_digit() && map[i + 1][j] != '.' as u8 {
        return true;
    }
    if i != map.len() - 1
        && j != map[i].len() - 1
        && !map[i + 1][j + 1].is_ascii_digit()
        && map[i + 1][j + 1] != '.' as u8
    {
        return true;
    }

    return false;
}

fn resolve_problem(input: &str) -> u32 {
    let mut map = Vec::new();
    input.lines().for_each(|line| map.push(line.as_bytes()));

    let mut result = 0;
    for (i, row) in map.iter().enumerate() {
        let numbers = get_numbers(row);

        for number in numbers {
            let mut j = number.start;
            while j < number.end {
                if check_symbol(&map, i, j as usize) {
                    result += number.value;
                    break;
                }
                j += 1;
            }
        }
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let output = resolve_problem(input);
        assert_eq!(4361, output);
    }
}
