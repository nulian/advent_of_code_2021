use super::super::file_reader;
use std::path::Path;

pub fn run_assignment() {
  let data = load_file_to_vector();
  part1(&data);
  part2(&data);
}

fn part1(data: &Vec<i32>) -> () {
  calculate_bigger(data);
}

fn part2(data: &Vec<i32>) -> () {
  let array_fetch_max = data.len() - 2;
  let mut n = 0;
  let mut combined_data: Vec<i32> = Vec::new();
  while n < array_fetch_max {
    combined_data.push(data[n..n+3].iter().sum());
    n += 1;
  }
  calculate_bigger(&combined_data);
}

fn calculate_bigger(data: &Vec<i32>) -> () {
  let mut total_bigger = 0;
  let mut previous_number: Option<i32> = None;
  for &number in data {
    if let Some(old_number) = previous_number {
      if number > old_number {
        total_bigger += 1;
      }
    }
    previous_number = Some(number);
  }
  println!("{}", total_bigger);
}

fn load_file_to_vector() -> Vec<i32> {
  let mut data: Vec<i32> = Vec::new();

  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment1.txt"));

  match file_data {
    Ok(lines) => {
      for line in lines {
        if let Ok(number) = line {
          data.push(number.parse().unwrap());
        }
      }
    }
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
  return data;
}
