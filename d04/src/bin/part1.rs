fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 04: Part 01 output: {output}\n");
}

#[derive(Debug)]
struct Card {
    _index: u32,
    winning: Vec<u8>,
    nums: Vec<u8>,
}

impl Card {
    fn from(line: &str) -> Self {
        Self {
            _index: line
                .split(':')
                .nth(0)
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),

            winning: line
                .split(':')
                .last()
                .unwrap()
                .trim()
                .split('|')
                .nth(0)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect(),

            nums: line
                .split(':')
                .last()
                .unwrap()
                .trim()
                .split('|')
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect(),
        }
    }
}

fn winnings(card: &Card) -> u32 {
    let wins = card
        .winning
        .iter()
        .enumerate()
        .map(|(i, w)| match card.nums.contains(w) {
            true => 1,
            _ => 0,
        })
        .sum::<u32>();

    if wins != 0 {
        return 2_u32.pow(wins - 1);
    }

    0
}

fn process(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let card = Card::from(line);
        let win = winnings(&card);
        sum += win;
    }

    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "13");
    }
}
