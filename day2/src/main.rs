mod game;

use aoc::AdventOfCodeDay;
use crate::game::{GameBag, GameSession};

fn main() {
    println!("Part 1: {}", Day2::part1());
    println!("Part 2: {}", Day2::part2());
}

struct Day2;

impl Day2 {
    fn get_games_from_file() -> Vec<GameSession> {
        let mut game_sessions = vec![];
        for line in std::fs::read_to_string(Self::FILENAME).unwrap().lines() {

            // Each line looks something like this:
            // Game 1: 6 green, 3 blue; 3 red, 1 green; 4 green, 3 red, 5 blue

            // First split on the semicolon to get the id and the bag contents
            let (game, contents) = line.split_once(": ").unwrap();
            let (_, id_str) = game.split_once(" ").unwrap();
            let id = id_str.parse::<u32>().unwrap();

            // Then split the contents by semicolon (each game entry)
            let mut game_session_bags = vec![];
            for bag in contents.split("; ") {
                // Then split the bag at the commas (each bag entry)
                let mut new_bag = GameBag::new();
                for entry in bag.split(", ") {
                    let (count, color) = entry.split_once(" ").unwrap();
                    new_bag.add(color.to_string(), count.parse::<u32>().unwrap());
                }

                game_session_bags.push(new_bag);
            }

            let game_session = GameSession::new(id, game_session_bags);
            game_sessions.push(game_session);
        }

        game_sessions
    }
}

impl AdventOfCodeDay for Day2 {
    const FILENAME: &'static str = "input.txt";
    type Part1Type = u32;
    type Part2Type = u32;

    fn part1() -> Self::Part1Type {
        let mut input_bag = GameBag::new();
        input_bag.add("red".to_string(), 12);
        input_bag.add("green".to_string(), 13);
        input_bag.add("blue".to_string(), 14);

        Self::get_games_from_file()
            .into_iter()
            .filter(|game_session| game_session.is_possible(&input_bag))
            .map(|game_session| game_session.id)
            .sum()
    }

    fn part2() -> Self::Part2Type {
        Self::get_games_from_file()
            .into_iter()
            .map(|game_session| game_session.part_2_power())
            .sum()
    }
}