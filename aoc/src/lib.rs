pub trait AdventOfCodeDay {
    fn part1<T>(&self) -> T;
    fn part2<T>(&self) -> T;
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
