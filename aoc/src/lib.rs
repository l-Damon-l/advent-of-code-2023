pub trait AdventOfCodeDay {

    const FILENAME: &'static str;

    type Part1Type : std::fmt::Display;
    type Part2Type : std::fmt::Display;

    fn part1() -> Self::Part1Type;
    fn part2() -> Self::Part2Type;

    fn print_answers() {
        println!("Part 1: {}", Self::part1());
        println!("Part 2: {}", Self::part2());
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
