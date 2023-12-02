use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 01: Part 02 output: {output}\n");
}

fn process(input: &str) -> String {
    let mut sum = 0;

    let words = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    for line in input.split_whitespace() {
        let mut digits : Vec<i32> = Vec::new();

        for (i, c) in line.chars().enumerate() {

            for (word, value) in words.iter() {
                if word.len() != 1 {
                    if word.len() + i  <= line.len() && &line[i..i + word.len()] == *word {
                        digits.push(*value);
                    }
                } else {
                    if c == word.chars().nth(0).unwrap() {
                        digits.push(*value);
                    }
                }
            }
        }

        let dig1 = digits.iter().nth(0).unwrap();
        let dig2 = digits.iter().last().unwrap();

        sum += dig1 * 10 + dig2;
    }

    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process(
            "two1nine\n \
            eightwothree\n \
            abcone2threexyz\n \
            xtwone3four\n \
            4nineeightseven2\n \
            zoneight234\n \
            7pqrstsixteen");
        assert_eq!(result, "281");
    }
}