use aoc::AdventOfCodeDay;
use std::collections::HashSet;

fn main() {
    Day4::print_answers();
}

struct Day4;

impl Day4 {
    fn parse_input() -> WinningNumbersAndCards {
        let file = std::fs::read_to_string(Self::FILENAME).expect("Error reading file");

        let mut winning_numbers_and_cards: Vec<WinningNumbersAndCard> = Vec::new();

        for line in file.lines() {
            // line looks like
            // Card   1: 10  5 11 65 27 43 44 29 24 69 | 65 66 18 14 17 97 95 34 38 23 10 25 22 15 87  9 28 43  4 71 89 20 72  5  6

            // split at | to get winning numbers and card
            let (winning_numbers, card_numbers) = line.split_once('|').unwrap();

            // remove the card number from the start
            let winning_numbers = winning_numbers.split_once(':').unwrap().1;

            let winning_numbers = winning_numbers
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();

            let card_numbers = card_numbers
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let winning_numbers_and_card =
                WinningNumbersAndCard::new(winning_numbers, card_numbers);
            winning_numbers_and_cards.push(winning_numbers_and_card);
        }

        WinningNumbersAndCards {
            number_sets: winning_numbers_and_cards,
        }
    }
}

impl AdventOfCodeDay for Day4 {
    const FILENAME: &'static str = "input.txt";
    type Part1Type = u32;
    type Part2Type = u32;

    fn part1() -> Self::Part1Type {
        Self::parse_input().sum_scores()
    }

    fn part2() -> Self::Part2Type {
        Self::parse_input().count_all_card_duplications()
    }
}

struct WinningNumbersAndCard {
    winning_numbers: HashSet<u32>,
    card_numbers: Vec<u32>,
    times: u32,
}

impl WinningNumbersAndCard {
    fn new(winning_numbers: HashSet<u32>, card_numbers: Vec<u32>) -> Self {
        Self {
            winning_numbers,
            card_numbers,
            times: 1,
        }
    }

    fn get_score(&self) -> u32 {
        let mut score = 0;

        for card_number in self.card_numbers.iter() {
            if self.winning_numbers.contains(&card_number) {
                score = if score == 0 { 1 } else { score * 2 };
            }
        }

        score
    }

    fn get_num_matches(&self) -> u32 {
        self.card_numbers
            .iter()
            .filter(|card_number| self.winning_numbers.contains(card_number))
            .count() as u32
    }

    fn add_duplications(&mut self, num_dupes: u32) {
        self.times += num_dupes;
    }
}

struct WinningNumbersAndCards {
    number_sets: Vec<WinningNumbersAndCard>,
}

impl WinningNumbersAndCards {
    fn sum_scores(&self) -> u32 {
        self.number_sets
            .iter()
            .map(|winning_numbers_and_card| winning_numbers_and_card.get_score())
            .sum()
    }

    fn count_all_card_duplications(&mut self) -> u32 {
        let mut total = 0u32;
        for i in 0..self.number_sets.len() {
            total += self.number_sets[i].times;
            let num_matches = self.number_sets[i].get_num_matches();
            self.add_dupe_to_sets(i + 1, num_matches as usize, self.number_sets[i].times);
        }

        total
    }

    fn add_dupe_to_sets(&mut self, start_index: usize, num_sets: usize, num_dupes: u32) {
        for i in start_index..start_index + num_sets {
            self.number_sets[i].add_duplications(num_dupes);
        }
    }
}
