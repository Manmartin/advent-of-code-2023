fn find_section_end(vec: &Vec<&str>, start: usize) -> usize {
    let mut i = start;
    while i < vec.len() && vec[i] != "" {
        i += 1;
    }
    i
}

fn make_section(lines: &Vec<&str>, seeds: &mut Vec<i64>, section: &str) {
    let seed_to_soil_start: usize = lines.iter().position(|&x| x == section).unwrap() + 1;
    let seed_to_soil_end: usize = find_section_end(&lines, seed_to_soil_start);
    
    let mut i = seed_to_soil_start;
    let mut ranges: Vec<Vec<i64>> = Vec::new();
    while i < seed_to_soil_end {
        ranges.push(lines[i].split(" ").filter_map(|x| x.parse().ok()).collect());
        i += 1;
    }

    for seed in seeds.iter_mut() {
        for range in &ranges {
            if *seed >= range[1] && *seed < range[1] + range[2]  {
                *seed = range[0] + *seed - range[1];
                break;
            }
        }
    }

}

fn resolve_problem(input: &str) -> i64{
    
    let lines: Vec<&str>  = input.lines().collect();

    let mut seeds: Vec<i64> = lines[0].split(" ").filter_map(|x| x.parse().ok()).collect();
    //let j = 0;
    make_section(&lines, &mut seeds, "seed-to-soil map:");
    make_section(&lines, &mut seeds, "soil-to-fertilizer map:");
    make_section(&lines, &mut seeds, "fertilizer-to-water map:");
    make_section(&lines, &mut seeds, "water-to-light map:");
    make_section(&lines, &mut seeds, "light-to-temperature map:");
    make_section(&lines, &mut seeds, "temperature-to-humidity map:");
    make_section(&lines, &mut seeds, "humidity-to-location map:");

    let mut min = seeds[0];
    for seed in seeds {
        if seed  < min {
            min = seed;
        }
    }

    min
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
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = resolve_problem(input);
        assert_eq!(35, output);
    }
}
