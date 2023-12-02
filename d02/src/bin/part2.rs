fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Day 02: Part 02 output: {output}\n");
}

#[derive(Debug)]
struct Game {
    index: u32,
    records: Vec<GameRecord>
}

#[derive(Debug)]
struct GameRecord {
    red: u8,
    green: u8,
    blue: u8
}

impl Game {
    fn from(input: &str) -> Self {
        Game {
            index: input.split(':')
                .nth(0).unwrap()
                .split_whitespace().last().unwrap()
                .parse().unwrap(),

            records: input.split(':')
                .last().unwrap()
                .split(';')
                .map(|r|
                    GameRecord {
                        red: match r.contains("red") {
                            true => {
                                r.split(',').filter(|x| x.contains("red")).last().unwrap().split_whitespace().nth(0).unwrap().parse().unwrap()
                            },
                            false => 0
                        },
                        green: match r.contains("green") {
                            true => {
                                r.split(',').filter(|x| x.contains("green")).last().unwrap().split_whitespace().nth(0).unwrap().parse().unwrap()
                            },
                            false => 0
                        },
                        blue: match r.contains("blue") {
                            true => {
                                r.split(',').filter(|x| x.contains("blue")).last().unwrap().split_whitespace().nth(0).unwrap().parse().unwrap()
                            },
                            false => 0
                        },
                    })
                .collect()
        }
    }

    fn max_reds(&self) -> u8 {
        self.records.iter().max_by_key(|r| r.red).unwrap().red
    }

    fn max_greens(&self) -> u8 {
        self.records.iter().max_by_key(|r| r.green).unwrap().green
    }

    fn max_blues(&self) -> u8 {
        self.records.iter().max_by_key(|r| r.blue).unwrap().blue
    }
}

fn process(input: &str) -> String {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let game = Game::from(line);
        sum += game.max_reds() as u64 * game.max_greens() as u64 * game.max_blues() as u64;
    }

    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n \
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n \
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n \
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n \
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "2286");
    }
}
