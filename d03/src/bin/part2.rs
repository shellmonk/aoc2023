fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 03: Part 02 output: {output}\n");
}


fn process(input: &str) -> String {

    let mut sum = 0;

    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "");
        assert_eq!(result, "0");
    }
}