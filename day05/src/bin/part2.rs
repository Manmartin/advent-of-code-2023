use std::cmp;

#[derive(Debug)]
struct Interval {
    start: i64,
    len: i64,
    end: i64,
}

impl Interval {
    fn new(start: i64, len: i64) -> Self {

        Self { start, len, end: start + len - 1 }
    }

    fn intersection(&self, other: Self) -> Option<Self> {
        if self.start > other.end || other.start > self.end {
            return  None;
        }
        else {
            let start = cmp::max(self.start, other.start);
            let len = cmp::min(self.end, other.end);
            return Some(Self::new(start, len));
        }
    }

    fn is_inside(&self, other: Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn update_start(&self, start: i64) {
        self.start = start;
        self.end = start + self.len - 1
    }

}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
    fn ne(&self, other: &Self) -> bool {
        !(self.start == other.start && self.end == other.end)
    }
}

fn find_section_end(vec: &Vec<&str>, start: usize) -> usize {
    let mut i = start;
    while i < vec.len() && vec[i] != "" {
        i += 1;
    }
    i
}

fn make_section_range(lines: &Vec<&str>, seeds: &mut Vec<Interval>, section: &str) {
    let start: usize = lines.iter().position(|&x| x == section).unwrap() + 1;
    let end: usize = find_section_end(&lines, start);
    
    let mut i = start;
    let mut ranges: Vec<[Interval; 2]> = Vec::new();

    while i < end {
        let temp: Vec<i64> = lines[i].split(" ").filter_map(|x| x.parse().ok()).collect();
        let destination = Interval::new(temp[0], temp[2]);
        let source = Interval::new(temp[0], temp[2]);
        ranges.push([destination, source]);
        i += 1;
    }

    let mut j = 0;

    while j < seeds.len() {
        for range in &ranges {
            if let Some(intesec) = range[1].intersection(seeds[j]) {
                if  seeds[i] == intesec {
                    let start = seeds[j].start + (range[0].start - range[1].start);
                    seeds[j].update_start(start);
                }
                else if 
            }
        }
    }
}

fn resolve_problem(input: &str) -> i64{

    let lines: Vec<&str>  = input.lines().collect();

    let mut seeds: Vec<i64> = lines[0].split(" ").filter_map(|x| x.parse().ok()).collect();
    
    let mut new_seeds: Vec<_> = Vec::new();
    let mut i = 0;

    while i < seeds.len() - 1 {
        new_seeds.push(Interval::new(seeds[i], seeds[i + 1]));
        i += 2;
    }
    println!("{:?}", new_seeds);
    make_section_range(&lines, &mut new_seeds, "seed-to-soil map:");
    return 0;
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
        assert_eq!(46, output);
    }
}
