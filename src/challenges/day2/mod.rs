use num_derive::FromPrimitive;
use num_traits::ToPrimitive;

use crate::tools::fileparser;

pub fn puzzle() {
    let data = fileparser::read_file_string("src/challenges/day2/input.txt").unwrap();
    let score: i32 = data
        .split("\n")
        .map(|v| v.split(" ").map(|v| str_to_hand(v)).collect::<Vec<Hand>>())
        .map(|v| hand_to_score(v.first().unwrap(), v.last().unwrap()))
        .sum();
    println!("Part 1 Score: {:?}", score);

    let new_score: i32 = data
      .split("\n")
      .map(|v| {
        let split_v: Vec<&str> = v.split(" ").collect();
        let hand = split_v[0];
        let desired_outcome = split_v[1];
        return (hand, desired_outcome)
      })
      .map(|(hand, outcome)| calc_play(&str_to_hand(hand), &str_to_outcome(outcome)))
      .map(|(theirs, mine)| hand_to_score(&theirs, &mine))
      .sum();
      println!("Part 2 Score: {:?}", new_score);
}

#[derive(FromPrimitive, ToPrimitive)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum DesiredScore {
    Win,
    Lose,
    Draw, 
}

fn str_to_outcome(outcome: &str) -> DesiredScore {
  match outcome {
    "X" => DesiredScore::Lose,
    "Y" => DesiredScore::Draw,
    "Z" => DesiredScore::Win,
    _ => panic!("Invalid outcome")
  }
}
fn str_to_hand(v: &str) -> Hand {
    match v {
        "A" => Hand::Rock,
        "X" => Hand::Rock,
        "B" => Hand::Paper,
        "Y" => Hand::Paper,
        "C" => Hand::Scissors,
        "Z" => Hand::Scissors,
        _ => panic!("Invalid hand"),
    }
}


fn calc_play(theirs: &Hand, desired: &DesiredScore) -> (Hand, Hand) {
    match desired {
        DesiredScore::Win => {
            match theirs {
                Hand::Rock => (Hand::Rock, Hand::Paper),
                Hand::Paper => (Hand::Paper, Hand::Scissors),
                Hand::Scissors => (Hand::Scissors, Hand::Rock),
            }
        }
        DesiredScore::Lose => {
            match theirs {
                Hand::Rock => (Hand::Rock, Hand::Scissors),
                Hand::Paper => (Hand::Paper, Hand::Rock),
                Hand::Scissors => (Hand::Scissors, Hand::Paper),
            }
        }
        DesiredScore::Draw => {
            match theirs {
                Hand::Rock => (Hand::Rock, Hand::Rock),
                Hand::Paper => (Hand::Paper, Hand::Paper),
                Hand::Scissors => (Hand::Scissors, Hand::Scissors),
            }
        }
    }
}

fn hand_to_score(theirs: &Hand, mine: &Hand) -> i32 {
    let base = match theirs {
        Hand::Rock => match mine {
            Hand::Rock => 3,
            Hand::Paper => 6,
            Hand::Scissors => 0,
        },
        Hand::Paper => match mine {
            Hand::Rock => 0,
            Hand::Paper => 3,
            Hand::Scissors => 6,
        },
        Hand::Scissors => match mine {
            Hand::Rock => 6,
            Hand::Paper => 0,
            Hand::Scissors => 3,
        },
    };
    return base + ToPrimitive::to_i32(mine).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_puzzle() {
        assert_eq!(hand_to_score(&Hand::Rock, &Hand::Paper), 8);
        assert_eq!(hand_to_score(&Hand::Paper, &Hand::Rock), 1);
    }
}
