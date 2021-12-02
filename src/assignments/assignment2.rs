use super::super::file_reader;
use std::path::Path;

enum Direction {
  Up,
  Down,
  Forward,
}

struct Movement {
  direction: Direction,
  amount: i32,
}

pub fn run_assignment() {
  let data = load_file_to_vector();
  part1(&data);
  part2(&data);
}

fn part1(data: &Vec<Movement>) {
  let mut horizontal_position = 0;
  let mut depth = 0;

  for movement in data {
    match movement.direction {
      Direction::Up => depth -= movement.amount,
      Direction::Down => depth += movement.amount,
      Direction::Forward => horizontal_position += movement.amount,
    }
  }
  println!(
    "Assignment2 part 1 solution: {}",
    horizontal_position * depth
  );
}

fn part2(data: &Vec<Movement>) {
  let mut horizontal_position = 0;
  let mut depth = 0;
  let mut aim = 0;

  for movement in data {
    match movement.direction {
      Direction::Up => aim -= movement.amount,
      Direction::Down => aim += movement.amount,
      Direction::Forward => {
        horizontal_position += movement.amount;
        depth = depth + aim * movement.amount;
      }
    }
  }
  println!(
    "Assignment2 part 2 solution: {}",
    horizontal_position * depth
  );
}

fn load_file_to_vector() -> Vec<Movement> {
  let mut data: Vec<Movement> = Vec::new();

  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment2.txt"));

  match file_data {
    Ok(lines) => {
      for result_line in lines {
        if let Ok(line) = result_line {
          let split_data: Vec<&str> = line.split_whitespace().collect();
          let direction = match split_data[0] {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => Direction::Up,
          };
          let amount: i32 = split_data[1].parse::<i32>().unwrap();
          data.push(Movement { direction, amount });
        }
      }
    }
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
  return data;
}