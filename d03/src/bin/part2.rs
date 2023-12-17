use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 03: Part 02 output: {output}\n");
}


fn process(input: &str) -> String {

    let mut sum = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for row in input.split_whitespace() {
        let mut tmp_row = Vec::new();
        for chr in row.chars() {
            tmp_row.push(chr);
        }
        grid.push(tmp_row);
    }

    let mut gears: Vec<(usize, usize)> = Vec::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '*' {
                gears.push((row_idx, col_idx));
            }
        }
    }

    let extract_number = |row: usize, col: usize| -> u32 {
        let mut start = col;
        let mut sum = 0;
        let mut cursor = 0;

        while start != 0 && grid[row][start - 1].is_ascii_digit() {
            start -= 1;
        }

        let mut digits = Vec::new();

        while start < grid[row].len() && grid[row][start].is_ascii_digit() {
            let digit = grid[row][start].to_digit(10).unwrap();
            digits.push(digit);
            cursor += 1;
            start += 1;
        }

        for (i, digit) in digits.iter().rev().enumerate() {
            sum += digit * 10_u32.pow(i.try_into().unwrap());
        }

        sum
    };



    for (gear_row, gear_col) in gears {
        let mut set = HashSet::new();
        if grid[gear_row + 1][gear_col].is_ascii_digit() { set.insert(extract_number(gear_row + 1, gear_col)); };
        if grid[gear_row + 1][gear_col + 1].is_ascii_digit() { set.insert(extract_number(gear_row + 1, gear_col + 1)); };
        if grid[gear_row + 1][gear_col - 1].is_ascii_digit() { set.insert(extract_number(gear_row + 1, gear_col - 1)); };
        if grid[gear_row][gear_col + 1].is_ascii_digit() { set.insert(extract_number(gear_row, gear_col + 1)); };
        if grid[gear_row][gear_col - 1].is_ascii_digit() { set.insert(extract_number(gear_row, gear_col - 1)); };
        if grid[gear_row - 1][gear_col].is_ascii_digit() { set.insert(extract_number(gear_row - 1, gear_col)); };
        if grid[gear_row - 1][gear_col + 1].is_ascii_digit() { set.insert(extract_number(gear_row - 1, gear_col + 1)); };
        if grid[gear_row - 1][gear_col - 1].is_ascii_digit() { set.insert(extract_number(gear_row - 1, gear_col - 1)); };

        if set.len() > 1 {
            println!("found {:?} at {gear_row}, {gear_col}", set);
            let mut tmp_sum = 1;
            for elem in set {
                tmp_sum *= elem;
            }

            sum += tmp_sum;
        }
    }

    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..");
        assert_eq!(result, "467835");
    }
}