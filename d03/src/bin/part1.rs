

const ROW: usize = 140;
const COL: usize = 140;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 03: Part 01 output: {output}\n");
}

fn symbol_in_neighbors(matrix: &[[char; ROW]; COL], cursor_row: usize, cursor_col: usize) -> bool {

    let rows = matrix.len();
    let cols = matrix[0].len();

    println!("rows: {rows}, cols: {cols}");

    let cursor = matrix[cursor_row][cursor_col];

    // left
    if cursor_col > 0 && !matrix[cursor_row][cursor_col - 1].is_ascii_digit() && !matrix[cursor_row][cursor_col - 1].is_whitespace() && matrix[cursor_row][cursor_col - 1] != '.' {
        println!("cursor {} left -> {}", cursor, matrix[cursor_row][cursor_col - 1]);
        return true;
    }

    // up left
    if cursor_col > 0 && cursor_row > 0 && !matrix[cursor_row - 1][cursor_col - 1].is_ascii_digit() && !matrix[cursor_row - 1][cursor_col - 1].is_whitespace() && matrix[cursor_row - 1][cursor_col - 1] != '.' {
        println!("cursor {} up left -> {}", cursor, matrix[cursor_row - 1][cursor_col - 1]);
        return true;
    }

    // up
    if cursor_row > 0 && !matrix[cursor_row - 1][cursor_col].is_ascii_digit() && !matrix[cursor_row - 1][cursor_col].is_whitespace() && matrix[cursor_row - 1][cursor_col] != '.' {
        println!("cursor {} up -> {}", cursor, matrix[cursor_row - 1][cursor_col]);
        return true;
    }

    // up right
    if cursor_row > 0 && cursor_col + 1 < cols && !matrix[cursor_row - 1][cursor_col + 1].is_ascii_digit() && !matrix[cursor_row - 1][cursor_col + 1].is_whitespace() && matrix[cursor_row - 1][cursor_col + 1] != '.' {
        println!("cursor {} up right -> {}", cursor, matrix[cursor_row - 1][cursor_col + 1]);
        return true;
    }

    // right
    if cursor_col + 1 < cols && !matrix[cursor_row][cursor_col + 1].is_ascii_digit() && !matrix[cursor_row][cursor_col + 1].is_whitespace() && matrix[cursor_row][cursor_col + 1] != '.' {
        println!("cursor {} right -> {}", cursor, matrix[cursor_row][cursor_col + 1]);
        return true;
    }

    // down right
    if cursor_row + 1 < rows && cursor_col + 1 < cols && !matrix[cursor_row + 1][cursor_col + 1].is_ascii_digit() && !matrix[cursor_row + 1][cursor_col + 1].is_whitespace() && matrix[cursor_row + 1][cursor_col + 1] != '.' {
        println!("cursor {} down right -> {}", cursor, matrix[cursor_row + 1][cursor_col + 1]);
        return true;
    }

    // down
    if cursor_row + 1 < rows && !matrix[cursor_row + 1][cursor_col].is_ascii_digit() && !matrix[cursor_row + 1][cursor_col].is_whitespace() && matrix[cursor_row + 1][cursor_col] != '.' {
        println!("cursor {} down -> {}", cursor, matrix[cursor_row + 1][cursor_col]);
        return true;
    }

    // down left
    if cursor_row + 1 < rows && cursor_col > 0 && !matrix[cursor_row + 1][cursor_col - 1].is_ascii_digit() && !matrix[cursor_row + 1][cursor_col - 1].is_whitespace() && matrix[cursor_row + 1][cursor_col - 1] != '.' {
        println!("cursor {} down left -> {}", cursor, matrix[cursor_row + 1][cursor_col - 1]);
        return true;
    }

    return false;
}

fn process(input: &str) -> String {

    let mut sum = 0;

    let mut grid = [[' ' as char; ROW]; COL];

    for (i, row) in input.split_whitespace().enumerate() {
        for (j, col) in row.chars().enumerate() {
            grid[i][j] = col;
        }
    }

    for (i, row) in input.split_whitespace().enumerate() {
        let mut digits: Vec<u32> = Vec::new();
        let mut open = false;
        let mut num_dirty = false;
        let mut num  = 0;

        let clean_row = row.trim();

        for (j, cursor) in clean_row.chars().enumerate() {

            if cursor.is_ascii_digit() {
                if !open {
                    open = true;
                }
                digits.push(cursor.to_digit(10).unwrap());

                if !num_dirty {
                    num_dirty = symbol_in_neighbors(&grid, i, j);
                }

                if j == COL - 1 {
                    for (deg, digit) in digits.iter().rev().enumerate() {
                        num += digit * 10_u32.pow(deg as u32);
                    }

                    println!("position: x {} ; y {} | {} | dirty: {} | sum: {}\n", i, j, num, num_dirty, sum);

                    if num_dirty {
                        sum += num;
                    }
                }

            } else {
                if open {

                    for (deg, digit) in digits.iter().rev().enumerate() {
                        num += digit * 10_u32.pow(deg as u32);
                    }

                    println!("position: x {} ; y {} | {} | dirty: {} | sum: {}\n", i, j, num, num_dirty, sum);

                    if num_dirty {
                        sum += num;
                    }

                    digits.clear();
                    open = false;
                    num_dirty = false;
                    num = 0;
                }
            }
        }


    }

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