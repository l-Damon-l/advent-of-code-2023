pub trait AdventOfCodeDay {

    const FILENAME: &'static str;

    type Part1Type;
    type Part2Type;

    fn part1() -> Self::Part1Type;
    fn part2() -> Self::Part2Type;
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
