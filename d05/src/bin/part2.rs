use std::collections::HashMap;
use std::cmp::min;

#[derive(Debug)]
struct Seed {
    start: u64,
    range: u64,
}

impl Seed {
    fn from(start: u64, range: u64) -> Self {
        Self { start, range }
    }
}

#[derive(Debug)]
struct Almanac {
    pub seeds: Vec<Seed>,
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

    fn parse_seeds(line: &str) -> Vec<Seed> {
        let mut seeds = Vec::new();

        let nums: Vec<u64> = line
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        for num in 0..nums.len() {
            if num % 2 == 0 {
                seeds.push(Seed::from(nums[num], nums[num + 1]))
            }
        }

        seeds
    }

    fn _get_relation_by_src_dst(&self, src: String, dst: String) -> &Relation {
        self.ranges
            .get_key_value(&Relation::from_src_dst(src, dst))
            .unwrap()
            .0
    }

    fn find_min(&self, path: &Vec<String>, seed: &Seed) -> u64 {
        let mut path_iter = path.iter().peekable();
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

        println!("{:?}", self.find_breaks((1, 1000), rel_path[0]));

        //let mut candidates = Vec::new();

        // for rel in rel_path {
        //     let r = self.get_ranges(0, 10000, rel);
        //     println!("{:?}", r);
        // }

        0
    }

    fn find_breaks(&self, segment: (u64, u64), relation: &Relation) -> Vec<(u64, u64)> {

        let find_dest_from_src = |src: u64, rec: &Record| -> u64 {
            rec.dst + (src - rec.src)
        };

        let calc_segments_for_record = |segment: (u64, u64), rec: &Record| -> Vec<(u64, u64)> {
            let mut tmp: Vec<(u64, u64)> = Vec::new();
            let seg_start = segment.0;
            let seg_end = segment.0 + segment.1;

            let rec_start = rec.src;
            let rec_end = rec.src + rec.range;

            if seg_start >= rec_start {
                if seg_end >= rec_end {
                    // TODO: WTF
                    tmp.push((find_dest_from_src(min(seg_end, rec_end), rec), rec.range));
                } else {
                    tmp.push((find_dest_from_src(seg_start, rec), segment.1));
                }
            } else {

            }

            tmp
        };

        self.ranges
            .get(relation)
            .unwrap()
            .iter()
            .filter(|rec|
                !(rec.src > segment.0 + segment.1 || rec.src + rec.range < segment.0))
            .flat_map(|rec| calc_segments_for_record(segment, rec))
            .collect::<Vec<(u64, u64)>>()
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

    let mut min = u64::MAX;

    for seed in &almanac.seeds {
        let val = almanac.find_min(&path, seed);
        println!("min for seed {:?} is {val}", seed);
        if val < min {
            min = val
        }
    }

    format!("{min}")
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
        assert_eq!(result, "46");
    }
}
