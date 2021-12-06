use super::super::file_reader;
use std::path::Path;


pub fn run_assignment() {
  let data = load_file_to_vector1();
  part1(&data);
  let data = load_file_to_vector2();

  part2(&data);
}

fn part2(data: &Vec<Vec<u8>>) {
  let mut process_data: Vec<Vec<u8>> = Vec::new();
  process_data.clone_from(data);
  let total_found = &data.into_iter().filter(|&elem| elem[0] == 0).count();
  let half_size = data.len() / 2;
  let mut most_common_bit = 0;
  if *total_found < half_size {
    most_common_bit = 1;
  }
  while process_data.len() > 1 {
    let filtered_data: Vec<Vec<u8>> = process_data.into_iter().filter(|elem| elem[0] != most_common_bit).collect();
    if filtered_data.len() > 0 {

    }
  }
  println!("{}", most_common_bit);
}

fn part1(data: &Vec<Vec<u8>>) {
  let mut binary: Vec<u8> = Vec::new();
  let mut binary2: Vec<u8> = Vec::new();
  for elem in data {
    let total_encounter = elem.into_iter().filter(|&elem| *elem == 0).count();
    let half_size = elem.len() / 2;
    if total_encounter > half_size {
      binary.push(0);
      binary2.push(1);
    } else {
      binary.push(1);
      binary2.push(0);
    }
  }
  let gamma: String = binary.iter().map(|&c| c.to_string()).collect::<String>();
  let epsilon: String = binary2.iter().map(|&c| c.to_string()).collect::<String>();
  let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
  let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();
  println!("{:?}", gamma_int * epsilon_int);
}
//
// fn part2(data: &Vec<Movement>) {
//   let mut horizontal_position = 0;
//   let mut depth = 0;
//   let mut aim = 0;
//
//   for movement in data {
//     match movement.direction {
//       Direction::Up => aim -= movement.amount,
//       Direction::Down => aim += movement.amount,
//       Direction::Forward => {
//         horizontal_position += movement.amount;
//         depth = depth + aim * movement.amount;
//       }
//     }
//   }
//   println!(
//     "Assignment2 part 2 solution: {}",
//     horizontal_position * depth
//   );
// }



fn load_file_to_vector1() -> Vec<Vec<u8>> {

  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment3.txt"));
  let peek_line = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment3.txt"));
  let column_size = peek_line.unwrap().peekable().peek().unwrap().as_ref().unwrap().chars().count();
  let mut data: Vec<Vec<u8>> = vec![Vec::new(); column_size];
  match file_data {
    Ok(lines) => {
      for result_line in lines {
        if let Ok(line) = result_line {
          for (i, char) in line.chars().enumerate() {
            data[i].push(char.to_digit(2).unwrap() as u8);
          }
        }
      }
    }
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
  return data;
}

fn load_file_to_vector2() -> Vec<Vec<u8>> {

  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment3.txt"));
  let mut data: Vec<Vec<u8>> = Vec::new();
  match file_data {
    Ok(lines) => {
      for result_line in lines {
        if let Ok(line) = result_line {
          let mut row_vec: Vec<u8> = Vec::new();
          for char in line.chars() {
            row_vec.push(char.to_digit(2).unwrap() as u8);
          }
          data.push(row_vec);
        }
      }
    }
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
  return data;
}
