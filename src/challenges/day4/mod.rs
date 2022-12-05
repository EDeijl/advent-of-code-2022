use itertools::Itertools;

use crate::tools::fileparser;

pub fn puzzle() {
  let data = fileparser::read_file_string("src/challenges/day4/input.txt").unwrap();
  let part_1 = data.split("\n")
    .map(|pair| pair.split(","))
    .map(|pair|  pair.map(|elf| { 
        let sectors: Vec<&str> = elf.split("-").collect();
        return (sectors[0].parse::<i32>().unwrap(), sectors[1].parse::<i32>().unwrap())
      }))
    .map(|pair| {
      let elfs: Vec<(i32, i32)> = pair.collect();
      (elfs[0].0 >= elfs[1].0 && elfs[0].1 <= elfs[1].1) || (elfs[1].0 >= elfs[0].0 && elfs[1].1 <= elfs[0].1)
    })
    .counts_by(|overlaps| overlaps);
  println!("Day 4 part 1: {:?}", part_1.get(&true).unwrap());

  let part_2 = data.split("\n")
    .map(|pair| pair.split(","))
    .map(|pair|  pair.map(|elf| { 
        let sectors: Vec<&str> = elf.split("-").collect();
        return (sectors[0].parse::<i32>().unwrap(), sectors[1].parse::<i32>().unwrap())
      }))
    .map(|pair| {
      let elfs: Vec<(i32, i32)> = pair.collect();
      (elfs[0].0 >= elfs[1].0 && elfs[0].0 <= elfs[1].1) || (elfs[1].0 >= elfs[0].0 && elfs[1].0 <= elfs[0].1)
    })
    .counts_by(|overlaps| overlaps);
  println!("Day 4 part 2: {:?}", part_2.get(&true).unwrap());
}