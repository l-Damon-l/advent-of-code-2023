use aoc::AdventOfCodeDay;
use std::collections::{HashMap, HashSet};

// This file is a mess...!

#[derive(Copy, Clone)]
struct PlanetPosition(usize, usize);

struct ExpandingRowAndCols {
    rows: HashSet<usize>,
    cols: HashSet<usize>,
}

struct Day11;

impl AdventOfCodeDay for Day11 {
    const FILENAME: &'static str = "input.txt";
    const TEST_FILENAME: Option<&'static str> = Some("test_input.txt");
    type Part1Type = u128;
    type Part2Type = u128;

    fn part1() -> Self::Part1Type {
        get_planet_pairs(&get_planet_positions(Day11::FILENAME, false))
            .into_iter()
            .map(|(p1, p2)| get_shortest_path(p1, p2))
            .sum()
    }

    fn part2() -> Self::Part2Type {
        get_planet_pairs(&get_planet_positions(Day11::FILENAME, true))
            .into_iter()
            .map(|(p1, p2)| get_shortest_path(p1, p2))
            .sum()
    }
}

fn get_planet_positions(filename: &str, is_part_2: bool) -> HashMap<u32, PlanetPosition> {
    let expanding_rows_and_cols = get_expanding_rows_and_cols(filename);

    let mut planet_positions: HashMap<u32, PlanetPosition> = HashMap::new();
    let mut planet_id = 0u32;

    let mut row_with_expansions = 0;
    for (i, line) in std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
    {
        // Expand the row if it's in the vector
        if expanding_rows_and_cols.rows.contains(&i) {
            row_with_expansions = if is_part_2 {
                row_with_expansions + 1_000_000
            } else {
                row_with_expansions + 2
            };

            // Nothing else to check after expanding the row
            continue;
        }

        let mut col = 0;
        let mut col_with_expansions = 0;
        while let Some(c) = line.chars().nth(col) {
            // Expand the columns if it's in the vector
            if expanding_rows_and_cols.cols.contains(&col) {
                col_with_expansions = if is_part_2 {
                    col_with_expansions + 999_999
                } else {
                    col_with_expansions + 1
                };
            }

            if c == '#' {
                planet_positions.insert(
                    planet_id,
                    PlanetPosition(row_with_expansions, col_with_expansions),
                );
                planet_id += 1;
            }

            col += 1;
            col_with_expansions += 1;
        }

        row_with_expansions += 1;
    }

    planet_positions
}

fn get_planet_pairs(
    planet_positions: &HashMap<u32, PlanetPosition>,
) -> Vec<(PlanetPosition, PlanetPosition)> {
    let mut planet_pairs: Vec<(PlanetPosition, PlanetPosition)> = Vec::new();

    for i in 0u32..planet_positions.len() as u32 - 1 {
        for j in i + 1..planet_positions.len() as u32 {
            let planet1 = *planet_positions.get(&i).unwrap();
            let planet2 = *planet_positions.get(&j).unwrap();
            planet_pairs.push((planet1, planet2));
        }
    }

    planet_pairs
}

fn get_expanding_rows_and_cols(filename: &str) -> ExpandingRowAndCols {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();

    let file = std::fs::read_to_string(filename).unwrap();

    // for rows that contain only '.' characters, add that row number to the rows vector
    file.lines().enumerate().for_each(|(i, line)| {
        if line.chars().all(|c| c == '.') {
            rows.insert(i);
        }
    });

    // for cols that contain only '.' characters, add that col number to the cols vector
    // get the length of the first lines
    for i in 0..file.lines().next().unwrap().len() {
        if file
            .lines()
            .map(|line| line.chars().nth(i).unwrap())
            .all(|c| c == '.')
        {
            cols.insert(i);
        }
    }

    ExpandingRowAndCols { rows, cols }
}

fn get_shortest_path(planet1: PlanetPosition, planet2: PlanetPosition) -> u128 {
    let x_diff = planet1.0.abs_diff(planet2.0);
    let y_diff = planet1.1.abs_diff(planet2.1);

    (x_diff + y_diff) as u128
}

fn main() {
    Day11::print_answers();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fn(is_part_2: bool) -> u128 {
        get_planet_pairs(&get_planet_positions(
            Day11::TEST_FILENAME.unwrap(),
            is_part_2,
        ))
        .iter()
        .map(|(p1, p2)| get_shortest_path(*p1, *p2))
        .sum()
    }

    #[test]
    fn part1_test() {
        assert_eq!(test_fn(false), 374);
    }

    #[test]
    fn part2_test() {
        assert_eq!(test_fn(true), 82000210);
    }
}
