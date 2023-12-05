fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 03: Part 01 output: {output}\n");
}


fn process(input: &str) -> String {

    let mut sum = 0;

    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "467..114..\n \
            ...*......\n \
            ..35..633.\n \
            ......#...\n \
            617*......\n \
            .....+.58.\n \
            ..592.....\n \
            ......755.\n \
            ...$.*....\n \
            .664.598..");
        assert_eq!(result, "4361");
    }

    #[test]
    fn it_should_work() {
        let result = process(
            "12.......*..\n \
            +.........34\n \
            .......-12..\n \
            ..78........\n \
            ..*....60...\n \
            78..........\n \
            .......23...\n \
            ....90*12...\n \
            ............\n \
            2.2......12.\n \
            .*.........*\n \
            1.1.......56");
        assert_eq!(result, "413");
    }
}