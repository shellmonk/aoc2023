fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 01: Part 02 output: {output}\n");
}

fn process(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process("");
        assert_eq!(result, "");
    }
}