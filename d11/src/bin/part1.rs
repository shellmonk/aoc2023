use itertools::Itertools;

fn process(input: &str) -> String {

    let mut sum = 0;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut stars: Vec<(usize, usize)> = Vec::new();

    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, chr)| {
            if *chr == '#' {
                stars.push((i, j));
            }
        })
    });

    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    for i in 0..map.len() {
        if stars.iter().filter(|(row, col)| { *row == i }).count() == 0 {
            empty_rows.push(i);
        }
    }

    for i in 0..map[0].len() {
        let found = map.iter().any(|row| row[i] == '#');
        if !found {
            empty_cols.push(i);
        }
    }


    stars.iter().combinations(2).for_each(|pair| {
        let s1 = *pair[0];
        let s2 = *pair[1];

        let row_delta = s1.0.abs_diff(s2.0) + empty_rows.iter().filter(|row| if s1.0 > s2.0 { **row < s1.0 && **row > s2.0 } else { **row > s1.0 && **row < s2.0 } ).count();
        let col_delta = s1.1.abs_diff(s2.1) + empty_cols.iter().filter(|col| if s1.1 > s2.1 { **col < s1.1 && **col > s2.1 } else { **col > s1.1 && **col < s2.1 } ).count();

        let delta = row_delta + col_delta;

        sum += delta;
        // println!("from {:?} to {:?}: {}", pair[0], pair[1], delta);
    });

    format!("{sum}")
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
            "...#......\n\
            .......#..\n\
            #.........\n\
            ..........\n\
            ......#...\n\
            .#........\n\
            .........#\n\
            ..........\n\
            .......#..\n\
            #...#.....",
        );
        assert_eq!(result, "374");
    }
}
