fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 01: Part 01 output: {output}\n");
}

fn process(input: &str) -> String {
    let mut sum = 0;
    for line in input.split_whitespace() {

        let digits = line.chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        let dig1 = digits.iter().nth(0).unwrap();
        let dig2 = digits.iter().last().unwrap();
        //
        // let mut first_set = false;
        // let mut dig1 = '0';
        // let mut dig2 = '0';
        // for c in line.chars() {
        //     if c.is_digit(10) {
        //         if first_set {
        //             dig2 = c;
        //         } else {
        //             dig1 = c;
        //             dig2 = c;
        //             first_set = true;
        //         }
        //     }
        // }
        sum += dig1 * 10 + dig2;
    }

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