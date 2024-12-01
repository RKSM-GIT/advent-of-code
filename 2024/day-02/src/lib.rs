pub fn process_part1(input: &str) -> String {
    "works".to_string()
}

pub fn process_part2(input: &str) -> String {
    "works".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("./sample_input.txt").unwrap();
        let res = process_part1(&input);
        println!("Result 1: {res}");
        assert_eq!(res, "11");
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("./sample_input.txt").unwrap();
        let res = process_part2(&input);
        println!("Result 2: {res}");
        assert_eq!(res, "31");
    }
}