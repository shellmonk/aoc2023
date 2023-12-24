use itertools::Itertools;

fn process(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let count = count_arrangements(&line.trim().to_string());
        println!("{line} : {count}");
        sum += count;
    }

    format!("{sum}")
}

fn count_arrangements(input: &String) -> u32 {
    if !input.contains("?") {
        return 1;
    };

    println!("Processing: {input}");
    let unknowns = input.chars().filter(|c| *c == '?').count();

    let groups: Vec<usize> = input.split(' ').collect::<Vec<&str>>()[1]
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| e.parse::<usize>().unwrap())
        .collect();

    let calc_groups = |input: &str| -> Vec<usize> {
        let binding = input.split(' ').collect::<Vec<&str>>()[0]
            .split(".")
            .collect::<Vec<&str>>();
        return binding
            .iter()
            .filter(|e| !e.is_empty())
            .map(|e| e.len())
            .collect();
    };

    let mut arrangements = 0;
    for x in (0..unknowns)
        .map(|_| vec!['.', '#'])
        .multi_cartesian_product()
    {
        let mut cnt = 0;
        let mut final_str_vec = Vec::new();
        for c in input.split(' ').collect::<Vec<&str>>()[0].chars() {
            if c == '?' {
                final_str_vec.push(x[cnt]);
                cnt += 1;
            } else {
                final_str_vec.push(c);
            }
        }

        let final_str = final_str_vec.iter().collect::<String>();
        if groups.eq(&calc_groups(final_str.as_str())) {
            // println!("{final_str:?}");
            arrangements += 1;
        }
    }
    arrangements
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 12: Part 01 output: {output}\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "???.### 1,1,3\n\
            .??..??...?##. 1,1,3\n\
            ?#?#?#?#?#?#?#? 1,3,1,6\n\
            ????.#...#... 4,1,1\n\
            ????.######..#####. 1,6,5\n\
            ?###???????? 3,2,1",
        );
        assert_eq!(result, "21");
    }
}
