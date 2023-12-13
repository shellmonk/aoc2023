fn solver(values: &Vec<i32>) -> i32 {
    if values.iter().sum::<i32>() == 0 {
        return 0
    }

    let mut tmps: Vec<i32> = Vec::new();

    for i in (0..values.len() - 1).rev() {
        let delta = values[i + 1] - values[i];
        tmps.push(delta)
    }
    tmps.reverse();

    values.first().unwrap() - solver(&tmps)

}

fn process(input: &str) -> String {
    let mut sum = 0;

    let solutions: Vec<_> = input.lines().map(|line| {
        let values: Vec<i32> = line.split_whitespace().collect::<Vec<_>>().iter().map(|val| val.parse::<i32>().unwrap()).collect();
        solver(&values)
    }).collect();

    for s in solutions {
        sum += s;
    }

    format!("{sum}")
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 09: Part 02 output: {output}\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45",
        );
        assert_eq!(result, "2");
    }
}
