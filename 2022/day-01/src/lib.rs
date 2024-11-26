use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn process_part1(input: &str) -> String {
    input
        .split("\r\n\r\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut heap = BinaryHeap::new();

    for elf_load in input.split("\r\n\r\n") {
        let sum = elf_load
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .sum::<i32>();

        heap.push(Reverse(sum));
        if heap.len() > 3 {
            heap.pop().unwrap();
        }
    }

    heap.iter().map(|x| x.0).sum::<i32>().to_string()
}