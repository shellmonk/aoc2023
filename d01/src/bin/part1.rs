fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 01: Part 01 output: {output}\n");
}

fn process(input: &str) -> String {
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.split_whitespace() {
        let mut first_set = false;
        let mut dig1 = '0';
        let mut dig2 = '0';
        for char in line.chars() {
            if char.is_digit(10) {
                if first_set {
                    dig2 = char;
                } else {
                    dig1 = char;
                    dig2 = char;
                    first_set = true;
                }
            }
        }
        numbers.push(format!("{}{}", dig1, dig2).parse::<i32>().unwrap());
    }

    let sum: i32 = numbers.iter().sum();
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "1abc2\n \
                  pqr3stu8vwx\n \
                  a1b2c3d4e5f\n \
                  treb7uchet");
        assert_eq!(result, "142");
    }
}