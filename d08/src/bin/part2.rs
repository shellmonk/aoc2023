use itertools::Itertools;
use std::collections::BTreeMap;

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

    let mut curr_loc = "AAA".to_string();

    let mut step = 0;
    while curr_loc != "ZZZ".to_string() {
        let inst = instructions.chars().cycle().nth(step).unwrap();
        let (lval, rval) = map.get(&curr_loc).unwrap();
        if inst == 'L' {
            curr_loc = lval.clone()
        } else {
            curr_loc = rval.clone();
        }

        step += 1;
    }

    format!("{step}")
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 08: Part 01 output: {output}\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "RL\n\
            \n\
            AAA = (BBB, CCC)\n\
            BBB = (DDD, EEE)\n\
            CCC = (ZZZ, GGG)\n\
            DDD = (DDD, DDD)\n\
            EEE = (EEE, EEE)\n\
            GGG = (GGG, GGG)\n\
            ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "2");
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
