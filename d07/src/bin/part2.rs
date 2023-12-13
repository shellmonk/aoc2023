use itertools::Itertools;
use std::cmp::Ordering;

fn process(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let (card, bid) = (
                line[0..5]
                    .chars()
                    .map(|c| match c {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'T' => 10,
                        '9' => 9,
                        '8' => 8,
                        '7' => 7,
                        '6' => 6,
                        '5' => 5,
                        '4' => 4,
                        '3' => 3,
                        '2' => 2,
                        'J' => 1,
                        _ => 0,
                    })
                    .collect::<Vec<u8>>(),
                line[6..].parse().unwrap(),
            );
            (card, bid)
        })
        .collect::<Vec<(Vec<u8>, u32)>>()
        .iter()
        .sorted_by(|c1, c2| {
            //println!("SORTING: {:?} and {:?}", c1, c2);

            let is_five_of_a_kind = |cards: &Vec<u8>| -> (bool, u8) {
                (
                    cards
                        .iter().dedup().collect::<Vec<_>>().len() == 1,
                    6,
                )
            };

            let is_four_of_a_kind = |cards: &Vec<u8>| -> (bool, u8) {
                (
                    cards[0..4]
                        .iter()
                        .filter(|c| **c != cards[0])
                        .collect::<Vec<&u8>>()
                        .is_empty()
                        || cards[1..5]
                        .iter()
                        .filter(|c| **c != cards[1])
                        .collect::<Vec<&u8>>()
                        .is_empty(),
                    5,
                )
            };

            let is_full_house = |cards: &Vec<u8>| -> (bool, u8) {
                if cards[0] == cards[1] {
                    if cards[2] == cards[3] && cards[3] == cards[4] {
                        return (true, 4);
                    }
                }

                if cards[3] == cards[4] {
                    if cards[0] == cards[1] && cards[1] == cards[2] {
                        return (true, 4);
                    }
                }

                (false, 4)
            };

            let is_three_of_a_kind = |cards: &Vec<u8>| -> (bool, u8) {
                (
                    (cards[0] == cards[1] && cards[1] == cards[2])
                        || (cards[1] == cards[2] && cards[2] == cards[3])
                        || (cards[2] == cards[3] && cards[3] == cards[4]),
                    3,
                )
            };

            let is_two_pair = |cards: &Vec<u8>| -> (bool, u8) {
                if is_three_of_a_kind(cards).0 || is_four_of_a_kind(cards).0 || is_four_of_a_kind(cards).0 || is_three_of_a_kind(cards).0 { return (false, 2) };

                let a = cards[0] == cards[1] && cards[2] == cards[3];
                let b = cards[0] == cards[1] && cards[3] == cards[4];
                let c = cards[1] == cards[2] && cards[3] == cards[4];

                (a || b || c, 2)
            };

            let is_one_pair = |cards: &Vec<u8>| -> (bool, u8) {
                for i in 0..cards.len() {
                    for j in i + 1..cards.len() {
                        if cards[i] == cards[j] {
                            return (true, 1);
                        }
                    }
                }

                (false, 1)
            };

            let rank = |cards: &Vec<u8>| -> u8 {

                let c = cards.iter().sorted_by(|c1, c2| c2.cmp(c1)).map(|e| e.clone()).collect_vec();

                let (mut res, mut rank) = is_five_of_a_kind(&c);

                if res {
                    return rank;
                }

                (res, rank) = is_four_of_a_kind(&c);
                if res {
                    return rank;
                }

                (res, rank) = is_full_house(&c);
                if res {
                    return rank;
                }

                (res, rank) = is_three_of_a_kind(&c);
                if res {
                    return rank;
                }

                (res, rank) = is_two_pair(&c);
                if res {
                    return rank;
                }

                (res, rank) = is_one_pair(&c);
                if res {
                    return rank;
                }

                0
            };


            let max_rank = |cards: &Vec<u8>| -> u8 {
                //println!("CARDS: {:?}", cards);
                let mut max = rank(cards);

                let jokers = cards.iter().filter(|card| **card == 1_u8).count();

                if jokers == 0 {
                    return max;
                }
                //println!("cards: {:?} | jokers: {jokers}", cards);

                if jokers > 3 {
                    return 5;
                }

                let all_cards: Vec<u8> = vec![14, 13, 12, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

                all_cards.iter().combinations(jokers).for_each(|comb| {
                    //println!("COMB: {:?}", comb);
                    let mut tmp_cards: Vec<u8> = Vec::new();
                    let mut tmp_j = 0;
                    //println!("comb: {:?}", comb);
                    for i in 0..cards.len() {
                        if cards[i] == 1 {
                            tmp_cards.push(*comb[tmp_j]);
                            tmp_j += 1;
                        } else {
                            tmp_cards.push(cards[i]);
                        }
                    }
                    let tmp_rank = rank(&tmp_cards);

                    if tmp_rank > max {
                        max = tmp_rank;
                        //println!("cards: {:?} | max cards: {:?}", cards, tmp_cards);
                    }
                });

                println!("cards: {:?} | jokers: {jokers} | max: {max}", cards);

                return max;
            };

            match max_rank(&c1.0).cmp(&max_rank(&c2.0)) {
                Ordering::Equal => {
                    println!("equals: {:?}, {:?}", c1, c2);
                    for i in 0..5 {
                        if c1.0[i] != c2.0[i] {
                            return (*c1.0)[i].cmp(&(*c2.0)[i]);
                        }
                    }
                    Ordering::Equal
                }
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            }
        })
        .enumerate()
        .map(|(idx, card)| {
            //println!("idx: {idx}");
            let delta = (idx as u32 + 1) * card.1;
            delta
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    format!("{sum}")
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 07: Part 02 output: {output}\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = process(
            "32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483",
        );
        assert_eq!(result, "5905");
    }
}
