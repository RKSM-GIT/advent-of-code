use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn process_part1(input: &str) -> String {
    "works".to_string()
}

pub fn process_part2(input: &str) -> String {
    "works".to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("./sample_input.txt").unwrap();

        assert_eq!(process_part1(&input), "15");
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("./sample_input.txt").unwrap();

        assert_eq!(process_part2(&input), "");
    }
}