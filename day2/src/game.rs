use std::collections::{HashMap, HashSet};

pub struct GameSession {
    pub id: u32,
    bags: Vec<GameBag>,
}

impl GameSession {
    pub fn new(id: u32, bag: Vec<GameBag>) -> Self {
        GameSession {
            id,
            bags: bag,
        }
    }

    /// Takes in an input bag and returns true if the output of the games is possible given that bag, false otherwise.
    pub fn is_possible(&self, input_bag: &GameBag) -> bool {
        for bag in &self.bags {
            for (key, value) in &bag.bag {
                if let Some(input_value) = input_bag.get(key) {
                    if input_value < value {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }

    /// Returns the product of the maximum number of each color in the session bags.
    pub fn part_2_power(&self) -> u32 {
        // Get each unique color (key) in the session bags
        let unique_colours = self.bags.iter()
            .map(|bag| bag.bag.keys())
            .flatten()
            .collect::<HashSet<&String>>();

        // Get the maximum number of each color in the session bags
        let mut power = 0;
        for colour in unique_colours {
            let max = self.bags.iter()
                .map(|bag| bag.get(colour).unwrap_or(&0u32))
                .filter(|&count| *count > 0)
                .max()
                .unwrap();

            if power == 0 {
                power = *max;
            } else {
                power *= max;
            }
        }

        power
    }
}

pub struct GameBag {
    bag: HashMap<String, u32>,
}

impl GameBag {
    pub fn new() -> Self {
        GameBag {
            bag: HashMap::new()
        }
    }

    pub fn add(&mut self, key: String, value: u32) {
        if let Some(existing_value) = self.bag.get_mut(&key) {
            *existing_value += value;
        } else {
            self.bag.insert(key, value);
        }
    }

    pub fn get(&self, key: &str) -> Option<&u32> {
        self.bag.get(key)
    }
}