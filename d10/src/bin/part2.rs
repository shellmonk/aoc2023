fn process(input: &str) -> String {


    let mut map: Vec<Vec<char>> = Vec::new();
    let mut visit_map: Vec<Vec<usize>> = Vec::new();

    let mut init_row = 0;
    let mut init_col = 0;


    for (l_idx, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        let mut visit_row = Vec::new();
        for (c_idx, c) in line.chars().enumerate() {
            visit_row.push(0);
            row.push(c);
            if c == 'S' {
                init_row = l_idx;
                init_col = c_idx;
            }
        }
        visit_map.push(visit_row);
        map.push(row);
    }

    let get_next_from_s = |row: usize, col: usize| -> (usize, usize) {

        let up = map[row - 1][col];
        let right = map[row][col + 1];
        let down = map[row + 1][col];
        let left = map[row][col - 1];

        if up == '|' || up == '7' || up == 'F' {
            return (row - 1, col);
        }

        if right == '-' || right == '7' || right == 'J' {
            return (row, col + 1);
        }

        if down == '|' || down == 'L' || down == 'J' {
            return (row + 1, col);
        }

        if left == '-' || left == 'L' || left == 'F' {
            return (row, col - 1);
        }

        (0, 0)
    };


    let get_next = |prev_row: usize, prev_col: usize, row: usize, col: usize| -> (usize, usize) {

        match map[row][col] {
            '|' => {
                if (prev_row, prev_col) == (row - 1, col) {
                    (row + 1, col)
                } else {
                    (row - 1, col)
                }
            },
            '-' => {
                if (prev_row, prev_col) == (row, col - 1) {
                    (row, col + 1)
                } else {
                    (row, col - 1)
                }
            },
            'L' => {
                if (prev_row, prev_col) == (row - 1, col) {
                    (row, col + 1)
                } else {
                    (row - 1, col)
                }
            },
            'J' => {
                if (prev_row, prev_col) == (row - 1, col) {
                    (row, col - 1)
                } else {
                    (row - 1, col)
                }
            },
            '7' => {
                if (prev_row, prev_col) == (row, col - 1) {
                    (row + 1, col)
                } else {
                    (row, col - 1)
                }
            },
            'F' => {
                if (prev_row, prev_col) == (row, col + 1) {
                    (row + 1, col)
                } else {
                    (row, col + 1)
                }
            },

            _ => {
                (prev_row, prev_col)
            }
        }
    };

    let (mut curr_row, mut curr_col) = get_next_from_s(init_row, init_col);
    let mut curr = map[curr_row][curr_col];

    while curr != 'S' {
        let (next_row, next_col) = get_next(init_row, init_col, curr_row, curr_col);
        init_row = curr_row;
        init_col = curr_col;
        curr_row = next_row;
        curr_col = next_col;

        curr = map[curr_row][curr_col];
    }

    dbg!(visit_map);

    "1".to_string()

}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 10: Part 01 output: {output}\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "...........\n\
            .S-------7.\n\
            .|F-----7|.\n\
            .||.....||.\n\
            .||.....||.\n\
            .|L-7.F-J|.\n\
            .|..|.|..|.\n\
            .L--J.L--J.\n\
            ...........",
        );
        assert_eq!(result, "4");
    }
}
