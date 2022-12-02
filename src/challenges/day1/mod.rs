use crate::tools::fileparser;
pub fn day_1_part_1() {
  let data  = fileparser::read_file_string("src/challenges/day1/real_input.txt").unwrap();
  let result = highest_calories(data);
  println!("Highest calories: {:?}", result);
}

pub  fn day_1_part_2() {
  let data = fileparser::read_file_string("src/challenges/day1/real_input.txt").unwrap();
  let result = top_3_highest_calories(data);
  println!("Top 3 highest calories combined: {:?}", result);
}

fn highest_calories(data: String) -> Option<i32> {
  let calories_per_elf = calories_per_elf(data);
  return calories_per_elf.iter().max().map(|&i| i);
}

fn top_3_highest_calories(data: String) -> Option<i32> {
  let calories_per_elf = calories_per_elf(data);
  let mut sorted = calories_per_elf.clone();
  sorted.sort();
  let  top_three: Vec<i32> = sorted.iter().rev().take(3).map(|&i| i).collect();
  let summed: i32 = top_three.iter().sum();
  return Some(summed);

}

fn calories_per_elf(data: String) -> Vec<i32> {
    let calories_per_elf: Vec<i32> = data.split("\n\n").map(|item| {
      return item.split("\n").map(|i| { i.parse::<i32>().unwrap() }).sum();
      }).collect();
    calories_per_elf
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_highest_calories() {
    let data = fileparser::read_file_string("src/challenges/day1/test_input.txt").unwrap();
  assert_eq!(highest_calories(data), Some(24000));
  }
  #[test]
  fn test_top_three() {
    let data = fileparser::read_file_string("src/challenges/day1/test_input.txt").unwrap();
    assert_eq!(top_3_highest_calories(data), Some(45000));

  }
}