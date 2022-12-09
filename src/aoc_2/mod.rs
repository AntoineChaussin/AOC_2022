use crate::get_input;


pub fn aoc_2_1(){
    let input = get_input("resource/aoc_2/data.txt");

    let mut total_score: Score = 0;

    for line in input.iter() {
        let split: Vec<&str> = line.split(" ").collect();
        assert!(split.len() == 2);

        let me: RPS = split[1].into();
        let other: RPS = split[0].into();

        let score = me.play(&other);

        total_score += score;
    }
    
    println!("AOC-2-1 total score: {}", &total_score);
}

pub fn aoc_2_2(){
    let input = get_input("resource/aoc_2/data.txt");

    let mut total_score: Score = 0;

    for line in input.iter() {
        let split: Vec<&str> = line.split(" ").collect();
        assert!(split.len() == 2);

        let outcome: FightResult = split[1].into();
        let other: RPS = split[0].into();

        let score = outcome.score() + strategy(&other, &outcome).score();

        total_score += score;
    }
    
    println!("AOC-2-2 total score: {}", &total_score);
}

#[derive(PartialEq,Eq,Clone, Debug)]
enum RPS {
    Rock, Paper, Scissors
}

enum FightResult {
    Win, Lose, Draw
}

type Score = u64;

impl From<&str> for RPS {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => unreachable!()
        }
    }
}

impl RPS {
    fn fight(&self,other : &RPS) ->  FightResult {
        match (self,other) {
            (me,other) if me.beats() == *other => FightResult::Win,
            (me, other) if me == other  => FightResult::Draw,
            (_,_) => FightResult::Lose
        }
    }

    fn score(&self) -> Score {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }

    fn beats(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper
        }
    }

    fn is_beat_by(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock
        }
    }

    fn play(&self, other: &RPS) -> Score {
        self.score() + self.fight(&other).score()
    }
}

impl FightResult {
    fn score(&self) -> Score {
        match self {
            FightResult::Win => 6,
            FightResult::Draw => 3,
            FightResult::Lose => 0
        }
    }
}

impl From<&str> for FightResult {
    fn from(s: &str) -> Self {
        match s {
            "X" => FightResult::Lose,
            "Y" => FightResult::Draw,
            "Z" => FightResult::Win,
            _ => unreachable!()
        }
    }
}

/// Returns the move needed to reach outcome given other
fn strategy(other: &RPS, outcome: &FightResult) -> RPS {
    match (outcome, other) {
        (FightResult::Draw, o) => o.clone(),
        (FightResult::Lose, o) => o.beats(),
        (FightResult::Win, o) => o.is_beat_by()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    use super::RPS::{Rock,Paper,Scissors};

    #[test]
    fn test_rps(){
        let me = vec![RPS::Rock,RPS::Paper,RPS::Scissors];
        let other = vec![RPS::Rock,RPS::Paper,RPS::Scissors];

        let all_scores:Vec<Score> = me.iter().cartesian_product(other).map(|(m,o)| m.play(&o)).collect();

        itertools::assert_equal(all_scores, vec![4,1,7,8,5,2,3,9,6]);
    }

    #[test]
    fn test_strat(){
        let me = vec![FightResult::Win,FightResult::Draw,FightResult::Lose];
        let other = vec![RPS::Rock,RPS::Paper,RPS::Scissors];

        let all_scores:Vec<RPS> = me.iter().cartesian_product(other).map(|(outcome,other)| strategy(&other, outcome)).collect();

        itertools::assert_equal(all_scores, vec![Paper, Scissors, Rock, Rock, Paper, Scissors, Scissors, Rock, Paper]);
    }

    #[test]
    fn test_aoc_2_1(){
        aoc_2_1()
    }

    #[test]
    fn test_aoc_2_2(){
        aoc_2_2()
    }


}