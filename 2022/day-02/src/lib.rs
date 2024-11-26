const SHAPE_SCORE: [usize; 3] = [1, 2, 3];
const OUTCOME_SCORE: [usize; 3] = [0, 3, 6];
const OUTCOME_MATRIX: [[usize; 3]; 3] = [
    [1, 0, 2],
    [2, 1, 0],
    [0, 2, 1]
];

const WINNING_MOVE: [usize; 3] = [1, 2, 0];
const LOOSING_MOVE: [usize; 3] = [2, 0, 1];

fn get_move(letter: &str) -> Result<usize, ()> {
    match letter {
        "A" | "X" => Ok(0),
        "B" | "Y" => Ok(1),
        "C" | "Z" => Ok(2),
        _ => Err(()),
    }
}

pub fn process_part1(input: &str) -> String {
    // 0 -> rock, 1 -> paper, 2 -> scissor
    // 0 -> lose, 1 -> draw, 2 -> win
    let mut score = 0;

    for line in input.lines() {
        let moves = line.split_ascii_whitespace()
            .map(|x| get_move(x))
            .collect::<Vec<_>>();

        let [my_move, opp_move] = [moves[1].unwrap(), moves[0].unwrap()];
        let outcome = OUTCOME_MATRIX[my_move][opp_move];
        score += SHAPE_SCORE[my_move] + OUTCOME_SCORE[outcome];
    }

    score.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut score = 0;

    for line in input.lines() {
        let moves = line.split_ascii_whitespace()
            .map(|x| get_move(x))
            .collect::<Vec<_>>();

        let opp_move = moves[0].unwrap();
        let my_move = match moves[1].unwrap() {
            0 => LOOSING_MOVE[opp_move],
            1 => opp_move,
            2 => WINNING_MOVE[opp_move],
            _ => 3
        };
        let outcome = OUTCOME_MATRIX[my_move][opp_move];
        score += SHAPE_SCORE[my_move] + OUTCOME_SCORE[outcome];
    }

    score.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("./sample_input.txt").unwrap();
        let res = process_part1(&input);
        println!("result: {}", res);
        assert_eq!(res, "15");
    }

    // #[test]
    // fn part2_test() {
    //     let input = fs::read_to_string("./sample_input.txt").unwrap();
    //     let res = process_part2(&input);
    //     println!("result: {}", res);
    //     assert_eq!(res, "");
    // }
}