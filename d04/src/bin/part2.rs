fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 04: Part 02 output: {output}\n");
}

#[derive(Debug, Clone)]
struct Card {
    index: usize,
    winning: Vec<u8>,
    nums: Vec<u8>,
    quantity: usize,
}

impl Card {
    fn from(line: &str) -> Self {
        Self {
            index: line
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
            quantity: 1,
        }
    }
}

fn winning_indexes(card: &Card) -> Vec<usize> {
    let wins: usize = card
        .winning
        .iter()
        .map(|w| match card.nums.contains(w) {
            true => 1,
            _ => 0,
        })
        .sum();

    return (card.index + 1..card.index + 1 + wins).collect::<Vec<usize>>();
}

fn process(input: &str) -> String {
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        cards.push(Card::from(line))
    }

    for i in 0..cards.len() {
        let card = &cards[i];
        let q = card.quantity;
        let wins = winning_indexes(card);
        for w in wins {
            let wcard = &mut cards[w - 1];
            wcard.quantity += q;
        }
    }

    let sum: usize = cards.iter().map(|c| c.quantity).sum();

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
        assert_eq!(result, "30");
    }
}
