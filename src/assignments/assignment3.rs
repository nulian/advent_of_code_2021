use super::super::file_reader;
use std::path::Path;


pub fn run_assignment() {
  let data = load_file_to_vector();
  part1(&data);
  part2(&data);
}

fn part2(data: &Vec<Vec<u8>>) {
  let oxygen_binary = calculate_binary_based_on_common_value(data, true);
  let co2_binary = calculate_binary_based_on_common_value(data, false);
  let oxygen_number = convert_binary_string_to_integer(convert_vec8_to_binary(oxygen_binary));
  let co2_number = convert_binary_string_to_integer(convert_vec8_to_binary(co2_binary));
  println!("Answer part 2: {}", oxygen_number * co2_number);
}

fn calculate_binary_based_on_common_value(data: &Vec<Vec<u8>>, most_common: bool) -> Vec<u8> {
  let mut process_data: Vec<Vec<u8>> = Vec::new();
  process_data.clone_from(data);
  let mut index: usize = 0;
  let mut process_data_length = process_data.len();
  while process_data_length > 1 {
    let current_common = common_value(&process_data, index, most_common);
    process_data.retain(|elem| elem[index] == current_common);
    index += 1;
    process_data_length = process_data.len();
  }

  return process_data.get(0).unwrap().clone();
}

fn common_value(data: &Vec<Vec<u8>>, index: usize, most_common: bool) -> u8 {
  let total_found = data.iter().filter(|&elem| elem[index] == 0).count();
  let half_size: f32 = data.len() as f32 / 2.0;
  if most_common && total_found as f32 <= half_size {
    return 1;
  } else if !most_common && total_found as f32 > half_size {
    return 1;
  }

  return 0;
}

fn calculate_binary_based_on_common_value_no_drop(data: &Vec<Vec<u8>>, most_common: bool) -> Vec<u8> {
  let mut process_data: Vec<Vec<u8>> = Vec::new();
  process_data.clone_from(data);
  let mut calculated_binary: Vec<u8> = Vec::new();
  let column_width = process_data[0].len();
  for _ in 0..column_width {
    let current_column_most_common = common_value(&process_data, calculated_binary.len(), most_common);
    calculated_binary.push(current_column_most_common);
  }

  return calculated_binary;
}



fn convert_vec8_to_binary(binary_vec: Vec<u8>) -> String {
  return binary_vec.iter().map(|&c| c.to_string()).collect::<String>();
}

fn convert_binary_string_to_integer(binary_string: String) -> isize {
  return isize::from_str_radix(&binary_string, 2).unwrap();
}

fn part1(data: &Vec<Vec<u8>>) {
  let binary = calculate_binary_based_on_common_value_no_drop(data, true);
  let binary2 = calculate_binary_based_on_common_value_no_drop(data, false);
  let gamma: String = convert_vec8_to_binary(binary);
  let epsilon: String = convert_vec8_to_binary(binary2);
  let gamma_int = convert_binary_string_to_integer(gamma);
  let epsilon_int = convert_binary_string_to_integer(epsilon);
  println!("answer part1: {}", gamma_int * epsilon_int);
}

fn load_file_to_vector() -> Vec<Vec<u8>> {

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
