use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in input.lines() {
        let elements: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        left_list.push(elements[0]);
        right_list.push(elements[1]);
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    let mut res = 0;
    for (&num1, &num2) in left_list.iter().zip(&right_list) {
        res += num1.abs_diff(num2);
    }

    res.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in input.lines() {
        let elements: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        left_list.push(elements[0]);
        right_list.push(elements[1]);
    }

    let mut map = HashMap::new();

    for num in right_list {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut res = 0;

    for num in left_list {
        res += num * map.get(&num).unwrap_or(&0);
    }

    res.to_string()
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