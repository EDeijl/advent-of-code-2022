use itertools::Itertools;

use crate::tools::fileparser;


pub fn puzzle() {
    let items: String = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let data = fileparser::read_file_string("src/challenges/day3/input.txt").unwrap();
    let summed_prio: i32 = data.split("\n")
    .map(|rucksack| {
      let compartments = rucksack.split_at(rucksack.len()/2);
      (compartments.0, compartments.1)
    })
    .map(|(left, right)| find_common_item(left, right))
    .map(|item| item.map(|i| items.chars().position(|c| c == i)).unwrap())
    .map(|i| i.map(|v| v as i32 +1).unwrap())
    .sum();

    println!("Day 3 part 1: {:?}", summed_prio);

    let badge_stuff: i32 = data.split("\n")
      .tuples::<(_,_,_)>()
      .map(|(first, second, third)| find_common_item2(first, second, third))
      .map(|item| item.map(|i| items.chars().position(|c| c == i).unwrap() as i32 + 1).unwrap())
      .sum();
    println!("Day 3 part 2: {:?}", badge_stuff);
}

pub fn find_common_item2(first: &str, second: &str, third: &str) -> Option<char> {
  for c in first.chars() {
    if second.chars().any(|s| s == c) && third.chars().any(|t| t == c) {
      return Some(c);
    }
  }
  None
}

pub fn find_common_item(left: &str, right: &str) -> Option<char> {
    for c in left.chars() {
        if right.chars().any(|rc| rc == c) {
          return Some(c);
        }
    }
    None
}