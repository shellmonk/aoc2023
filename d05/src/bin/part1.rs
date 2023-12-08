use std::collections::HashMap;

#[derive(Debug)]
struct Almanac {
    pub seeds: Vec<u64>,
    pub ranges: HashMap<Relation, Vec<Record>>,
}

impl Almanac {
    fn from_input(input: &str) -> Self {
        let mut relation = Relation::from_src_dst("".to_string(), "".to_string());

        let mut almanac = Almanac {
            seeds: Vec::new(),
            ranges: HashMap::new(),
        };

        for (_, line) in input.lines().enumerate() {
            if line.starts_with("seeds") {
                almanac.seeds = Almanac::parse_seeds(line);
            } else if line.contains("map:") {
                relation = Relation::from_line(line);
                almanac.ranges.insert(relation.clone(), Vec::new());
            } else if !line.trim().is_empty() {
                let record = Record::from_line(line);
                let range = almanac.ranges.get_mut(&relation).unwrap();
                range.push(record);
            } else {
            }
        }

        almanac
    }

    fn parse_seeds(line: &str) -> Vec<u64> {
        line.split(':')
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect()
    }

    fn _get_relation_by_src_dst(&self, src: String, dst: String) -> &Relation {
        self.ranges
            .get_key_value(&Relation::from_src_dst(src, dst))
            .unwrap()
            .0
    }

    fn find(&self, path: &Vec<String>, start_val: u64) -> u64 {
        let mut path_iter = path.iter().peekable();
        let mut curr_val = start_val;

        let mut rel_path: Vec<&Relation> = Vec::new();

        while let Some(point) = path_iter.next() {
            if path_iter.peek().is_some() {
                rel_path.push(
                    self.ranges
                        .get_key_value(&Relation::from_src_dst(
                            point.clone(),
                            (*path_iter.peek().unwrap()).clone(),
                        ))
                        .unwrap()
                        .0,
                );
            }
        }



        for r in rel_path {
            let records = self.ranges.get(r).unwrap();

            for record in records {
                if curr_val >= record.src && curr_val <= record.src + record.range {
                    println!("found curr_val: {curr_val} in Record: {:?}", record);
                    curr_val = record.dst + (curr_val - record.src);
                    println!("new curr_val: {curr_val}");
                    break;
                }
            }

        }

        curr_val
    }
}

#[derive(Eq, PartialOrd, Ord, Hash, Debug)]
struct Relation {
    pub src: String,
    pub dst: String,
}

impl Relation {
    fn from_line(line: &str) -> Self {
        let mut t = line
            .split(':')
            .nth(0)
            .unwrap()
            .split_whitespace()
            .nth(0)
            .unwrap()
            .split("-to-");

        Self {
            src: t.nth(0).unwrap().to_string(),
            dst: t.last().unwrap().to_string(),
        }
    }
    fn from_src_dst(src: String, dst: String) -> Self {
        Self { src, dst }
    }
}

impl PartialEq for Relation {
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src && self.dst == other.dst
    }
}

impl Clone for Relation {
    fn clone(&self) -> Self {
        Relation {
            src: self.src.clone(),
            dst: self.dst.clone(),
        }
    }
}

#[derive(Debug)]
struct Record {
    src: u64,
    dst: u64,
    range: u64,
}

impl Record {
    fn from_line(line: &str) -> Record {
        let nums: Vec<u64> = line
            .trim()
            .split(' ')
            .map(|e| e.parse::<u64>().unwrap())
            .collect();

        Self {
            dst: nums.get(0).unwrap().clone(),
            src: nums.get(1).unwrap().clone(),
            range: nums.get(2).unwrap().clone(),
        }
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 05: Part 01 output: {output}\n");
}

fn process(input: &str) -> String {
    let mut solutions: Vec<u64> = Vec::new();
    let almanac = Almanac::from_input(input);

    let path = vec![
        "seed".to_string(),
        "soil".to_string(),
        "fertilizer".to_string(),
        "water".to_string(),
        "light".to_string(),
        "temperature".to_string(),
        "humidity".to_string(),
        "location".to_string(),
    ];

    for seed in &almanac.seeds {
        let s = almanac.find(&path, *seed);
        println!("seed: {}, {}", seed, s);
        solutions.push(s);
    }

    let v = solutions.iter().min().unwrap();

    format!("{v}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "seeds: 79 14 55 13\n
            \n
            seed-to-soil map:\n
            50 98 2\n
            52 50 48\n
            \n
            soil-to-fertilizer map:\n
            0 15 37\n
            37 52 2\n
            39 0 15\n
            \n
            fertilizer-to-water map:\n
            49 53 8\n
            0 11 42\n
            42 0 7\n
            57 7 4\n
            \n
            water-to-light map:\n
            88 18 7\n
            18 25 70\n
            \n
            light-to-temperature map:\n
            45 77 23\n
            81 45 19\n
            68 64 13\n
            \n
            temperature-to-humidity map:\n
            0 69 1\n
            1 0 69\n
            \n
            humidity-to-location map:\n
            60 56 37\n
            56 93 4",
        );
        assert_eq!(result, "35");
    }
}
