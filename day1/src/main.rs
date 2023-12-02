use std::collections::HashMap;
use aoc::AdventOfCodeDay;
use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBER_MAP: HashMap<&'static str, u32> = HashMap::from([
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
    ]);
}

struct Day1;

impl Day1 {

    /// Converts a string to a string where all number words (1 - 9 only) are replaced with their digit equivalents.
    ///
    /// # Arguments
    /// * `line` - A string slice to convert.
    /// # Returns
    /// A new string where all number words (1 - 9 only) are replaced with their digit equivalents.
    fn string_converter(line: &str) -> String {

        // Create a new string to hold the converted string
        let mut new_string = String::new();

        // Loop through the characters in the line
        let mut i = 0;
        while i < line.len() {

            // Create an iterator from the current position of slices of length 3, 4, and 5 (all number words are 3-5 characters long)
            let possible_num_strings = (3..=5)
                .map(|n| line.get(i..i + n))
                .into_iter()
                .filter_map(|s| s);

            // Loop through the possible number strings and see if any of them are in the number map...
            // If they are, add the number to the new string, otherwise, just add the character to the new string.
            // 'one' gets converted to 1ne, but this doesn't matter for the purposes of this problem.
            // If the characters that create a number get skipped, then something like oneight would be converted to 1ight rather than 18, so that won't work.
            // If it is converted instead to 1n8ight, that will be fine.
            let mut number_string_found = false;
            for possible_num_string in possible_num_strings {
                if let Some(number) = NUMBER_MAP.get(possible_num_string) {
                    new_string.push_str(number.to_string().as_str());
                    number_string_found = true; // don't add the character to the new string (might not actually matter)
                    break; // Having two different words at the same location is not possible.
                }
            }

            if !number_string_found {
                new_string.push(line.chars().nth(i).unwrap());
            }

            i += 1;
        }

        new_string
    }
}

impl AdventOfCodeDay for Day1 {
    const FILENAME: &'static str = "input.txt";
    type Part1Type = u32;
    type Part2Type = u32;

    fn part1() -> u32 {
        std::fs::read_to_string(Self::FILENAME)
            .expect(format!("Failed to read {}", Self::FILENAME).as_str())
            .lines()
            .map(line_to_number)
            .sum()
    }

    // The solution to this could be improved by iterating through the line and at each position checking
    // if it is a digit or if it is a number word rather than converting each line to new strings before then checking,
    // but it works well enough for this problem.
    fn part2() -> u32 {
        std::fs::read_to_string(Self::FILENAME)
            .expect(format!("Failed to read {}", Self::FILENAME).as_str())
            .lines()
            .map(|s| {
                let converted_string = Self::string_converter(s);
                line_to_number(&converted_string)
            })
            .sum()
    }
}

/// Converts a line like "adfd1djfhd2dj" to 12.
/// # Arguments
/// * `line` - A string slice that contains a number (if it doesn't, this function will panic).
fn line_to_number(line: &str) -> u32 {
    let first_digit = get_first_digit_as_u32(line.chars()).unwrap();
    let last_digit = get_first_digit_as_u32(line.chars().rev()).unwrap();
    (first_digit * 10) + last_digit
}

/// Finds the first digit in a given character iterator.
///
/// This function takes an iterator over characters and returns the first character that is a digit.
/// # Arguments
/// * `chars` - A mutable reference to an iterator over characters.
fn get_first_digit_as_u32(mut chars: impl Iterator<Item=char>) -> Option<u32> {
    chars.find(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap())
}

fn main() {
    println!("Part 1: {}", Day1::part1());
    println!("Part 2: {}", Day1::part2());
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    //noinspection ALL
    #[test_case("one", String::from("1ne"))]
    #[test_case("one68fdhjfhdtwo", String::from("1ne68fdhjfhd2wo"))]
    #[test_case("ttthreekjeightnine1ss", String::from("tt3hreekj8ight9ine1ss"))]
    #[test_case("onetwonine4noneightvk", String::from("1ne2wo9ine4n1n8ightvk"))]
    fn string_converter_tests(input: &str, expected: String) {
        assert_eq!(Day1::string_converter(input), expected);
    }

    //noinspection ALL
    #[test_case("one", 11)]
    #[test_case("t8t", 88)]
    #[test_case("two", 22)]
    #[test_case("one68fdhjfhdtwo", 12)]
    #[test_case("odfd68fdhjfhdtwo", 62)]
    #[test_case("ofeightdhjf45hd", 85)]
    #[test_case("6jnnmhkfourfive63eightggchjtwonet", 61)]
    #[test_case("onetwonine4noneightvk", 18)]
    fn line_to_number_with_converter_tests(input: &str, expected: u32) {
        let converted_string = Day1::string_converter(input);
        assert_eq!(line_to_number(&converted_string), expected);
    }
}