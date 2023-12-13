use itertools::Itertools;
use std::collections::BTreeMap;
use num::integer::lcm;

fn get_path_len(map: &BTreeMap<String, (String, String)>, init: &String, instructions: &String) -> u32 {
    let mut curr_loc = init.clone();
    let mut step = 0;

    while curr_loc.chars().last().unwrap() != 'Z' {
        let inst = instructions.chars().cycle().nth(step).unwrap();
        let (lval, rval) = map.get(&curr_loc).unwrap();
        if inst == 'L' {
            curr_loc = lval.clone()
        } else {
            curr_loc = rval.clone();
        }

        step += 1;
    }

    step as u32
}

fn process(input: &str) -> String {
    let mut map: BTreeMap<String, (String, String)> = BTreeMap::new();

    let instructions = input.lines().nth(0).unwrap().trim();

    for line in input.lines() {
        if line.contains("=") {
            let key = line[0..3].to_string();
            let lvalue = line[7..10].to_string();
            let rvalue = line[12..15].to_string();

            map.insert(key, (lvalue, rvalue));
        }
    }


    let mut tracks = Vec::new();

    map.iter().for_each(|e| {
        if e.0.chars().last().unwrap() == 'A' {
            tracks.push(e.0.clone());
        }
    });

    println!("TRACKS: {:?}", tracks);


    let path_lens = tracks.iter().map(|track| {
        get_path_len(&map, track, &instructions.to_string()) as u128
    }).collect_vec();


    let mut step = path_lens[0];
    for i in 1..path_lens.len() {
        step = lcm(step, path_lens[i]);
        println!("STEP: {step}");
    }

    format!("{step}")
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 08: Part 02 output: {output}\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "LR\n\
            \n\
            11A = (11B, XXX)\n\
            11B = (XXX, 11Z)\n\
            11Z = (11B, XXX)\n\
            22A = (22B, XXX)\n\
            22B = (22C, 22C)\n\
            22C = (22Z, 22Z)\n\
            22Z = (22B, 22B)\n\
            XXX = (XXX, XXX)",
        );
        assert_eq!(result, "6");
    }

    #[test]
    fn it_should_work_2() {
        let result = process(
            "RLL\n\
            \n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "6");
    }
}
