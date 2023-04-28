use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let input_file = File::open("input.txt")
        .expect("Expected input.txt file in project");
    let mut game_score = 0;
    for input in BufReader::new(input_file).lines() {
        let round: Vec<char> = input.unwrap().chars().collect();
        game_score += score_round2(round[2], round[0]);
    }
    print!("{}", game_score);
}

fn score_round1(my_move: char, opponent_move: char) -> i32 {
    let mut round_score = 0;
    if my_move == 'X' {
        if opponent_move == 'A' {
            round_score += 4;
        } else if opponent_move == 'B' {
            round_score += 1
        } else if opponent_move == 'C' {
            round_score += 7
        }
    } else if my_move == 'Y' {
        if opponent_move == 'A' {
            round_score += 8;
        } else if opponent_move == 'B' {
            round_score += 5
        } else if opponent_move == 'C' {
            round_score += 2
        }
    } else if my_move == 'Z' {
        if opponent_move == 'A' {
            round_score += 3;
        } else if opponent_move == 'B' {
            round_score += 9
        } else if opponent_move == 'C' {
            round_score += 6
        }
    }
    return round_score;
}

fn score_round2(result: char, opponent_move: char) -> i32 {
    let mut round_score = 0;
    if result == 'X' { //lose
        if opponent_move == 'A' {
            round_score += 3;
        } else if opponent_move == 'B' {
            round_score += 1
        } else if opponent_move == 'C' {
            round_score += 2
        }
    } else if result == 'Y' { //draw
        if opponent_move == 'A' {
            round_score += 4;
        } else if opponent_move == 'B' {
            round_score += 5
        } else if opponent_move == 'C' {
            round_score += 6
        }
    } else if result == 'Z' { //win
        if opponent_move == 'A' {
            round_score += 8;
        } else if opponent_move == 'B' {
            round_score += 9
        } else if opponent_move == 'C' {
            round_score += 7
        }
    }
    return round_score;
}
