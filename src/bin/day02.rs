use std::fs::read_to_string;

use Shapes::{Paper, Rock, Scissors};

use crate::Result::{Draw, Lose, Win};

enum Shapes {
    Rock,
    Paper,
    Scissors,
}

impl Shapes {
    fn score(shape: &Shapes) -> i32 {
        match shape {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
    fn play(me: &Shapes, opponent: &Shapes) -> i32 {
        match (me, opponent) {
            (Rock, Scissors) => 6,
            (Scissors, Paper) => 6,
            (Paper, Rock) => 6,
            (Rock, Rock) => 3,
            (Paper, Paper) => 3,
            (Scissors, Scissors) => 3,
            (Scissors, Rock) => 0,
            (Paper, Scissors) => 0,
            (Rock, Paper) => 0,
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut total_score = 0;

    for line in read_to_string("inputs/day02.txt").unwrap().lines() {
        let (opponent_shape, my_shape) = line.split_once(" ")
            .map(|(op, me)| {
                (
                    match op {
                        "A" => Rock,
                        "B" => Paper,
                        "C" => Scissors,
                        _ => panic!("Unknown play by opponent"),
                    },
                    match me {
                        "X" => Rock,
                        "Y" => Paper,
                        "Z" => Scissors,
                        _ => panic!("Unknown play by me"),
                    }
                )
            })
            .expect("Plays can be parsed");

        let shape_score = Shapes::score(&my_shape);
        let play_score = Shapes::play(&my_shape, &opponent_shape);

        total_score += shape_score + play_score;
    }

    println!("Part1: total score is {}", total_score);
}

enum Result {
    Lose,
    Draw,
    Win,
}

fn part2() {
    let mut total_score = 0;

    for line in read_to_string("inputs/day02.txt").unwrap().lines() {
        let (opponent_shape, result) = line.split_once(" ")
            .map(|(op, result)| {
                (
                    match op {
                        "A" => Rock,
                        "B" => Paper,
                        "C" => Scissors,
                        _ => panic!("Unknown play by opponent"),
                    },
                    match result {
                        "X" => Lose,
                        "Y" => Draw,
                        "Z" => Win,
                        _ => panic!("Unknown play by me"),
                    }
                )
            })
            .expect("Plays can be parsed");

        let my_shape = match (&opponent_shape, result) {
            (Rock, Lose) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Win) => Paper,
            (Paper, Lose) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Lose) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Win) => Rock,
        };

        let shape_score = Shapes::score(&my_shape);
        let play_score = Shapes::play(&my_shape, &opponent_shape);

        total_score += shape_score + play_score;
    }

    println!("Part2: total score is {}", total_score);
}
