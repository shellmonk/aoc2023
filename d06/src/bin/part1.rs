use std::borrow::Borrow;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 06: Part 01 output: {output}\n");
}

fn process(input: &str) -> String {
    let sum = 0;

    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "Time:      7  15   30\n\
            Distance:  9  40  200",
        );
        assert_eq!(result, "288");
    }
}
