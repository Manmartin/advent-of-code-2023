#[derive(Debug)]
struct Number {
    value: u32,
    start: u32,
    end: u32,
    row: u32,
}

fn get_number(map: &Vec<&[u8]>, i: usize, j: usize) -> Number {
    let mut start = j;
    let mut end = j;
    while start > 0 && map[i][start - 1].is_ascii_digit() {
        start -= 1
    }
    while end < map[i].len() && map[i][end].is_ascii_digit() {
        end += 1;
    }

    let value = std::str::from_utf8(&map[i][start..end]).unwrap();
    let value = value.parse().unwrap();

    Number {
        value,
        start: start as u32,
        end: end as u32,
        row: i as u32,
    }
}

fn check_gear(map: &Vec<&[u8]>, i: usize, j: usize) -> u32 {
    let mut result: Vec<Number> = Vec::new();

    if i != 0 && j != 0 && map[i - 1][j - 1].is_ascii_digit() {
        let n = get_number(map, i - 1, j - 1);
        if result
            .iter()
            .find(|&x| x.start == n.start && x.end == n.end && x.row == n.row)
            .is_none()
        {
            result.push(n);
        }
    }
    if i != 0 && map[i - 1][j].is_ascii_digit() {
        let n = get_number(map, i - 1, j);
        if result
            .iter()
            .find(|&x| x.start == n.start && x.end == n.end && x.row == n.row)
            .is_none()
        {
            result.push(n);
        }
    }
    if i != 0 && j != map[i].len() && map[i - 1][j + 1].is_ascii_digit() {
        let n = get_number(map, i - 1, j + 1);
        if result
            .iter()
            .find(|&x| x.start == n.start && x.end == n.end && x.row == n.row)
            .is_none()
        {
            result.push(n);
        }
    }

    if j != 0 && map[i][j - 1].is_ascii_digit() {
        let n = get_number(map, i, j - 1);
        if result
            .iter()
            .find(|&x| x.start == n.start && x.end == n.end && x.row == n.row)
            .is_none()
        {
            result.push(n);
        }
    }
    if j != map.len() - 1 && map[i][j + 1].is_ascii_digit() {
        let n = get_number(map, i, j + 1);
        if result
            .iter()
            .find(|&x| x.start == n.start && x.end == n.end && x.row == n.row)
            .is_none()
        {
            result.push(n);
        }
    }

    if i != map.len() - 1 && j != 0 && map[i + 1][j - 1].is_ascii_digit() {
        let n = get_number(map, i + 1, j - 1);
        if result
            .iter()
            .find(|&x| x.start == n.start && x.end == n.end && x.row == n.row)
            .is_none()
        {
            result.push(n);
        }
    }
    if i != map.len() - 1 && map[i + 1][j].is_ascii_digit() {
        let n = get_number(map, i + 1, j);
        if result
            .iter()
            .find(|&x| x.start == n.start && x.end == n.end && x.row == n.row)
            .is_none()
        {
            result.push(n);
        }
    }
    if i != map.len() - 1 && j != map[i].len() - 1 && map[i + 1][j + 1].is_ascii_digit() {
        let n = get_number(map, i + 1, j + 1);
        if result
            .iter()
            .find(|&x| x.start == n.start && x.end == n.end && x.row == n.row)
            .is_none()
        {
            result.push(n);
        }
    }
    if result.len() != 2 {
        return 0;
    }
    let mut number = 1;
    for n in result {
        number *= n.value;
    }
    return number;
}

fn resolve_problem(input: &str) -> u32 {
    let mut map = Vec::new();
    input.lines().for_each(|line| map.push(line.as_bytes()));

    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '*' as u8 {
                result += check_gear(&map, i, j);
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
        assert_eq!(467835, output);
    }
}
