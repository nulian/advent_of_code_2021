use super::super::file_reader;
use std::path::Path;
use std::cmp::{max, min};
pub fn run_assignment() {
  let mut pool = load_file_to_vector();
  part1(&mut pool);
  let mut pool2 = load_file_to_vector();
  part2(&mut pool2);
}

fn part1(calc: &mut Calc) {
  calc.calculate_consumptions(false);
  println!("result part1: {:?}", calc.find_lowest_consumption_target());
}
fn part2(calc: &mut Calc) {
  calc.calculate_consumptions(true);
  println!("result part2: {:?}", calc.find_lowest_consumption_target());
}
#[derive(Debug)]
struct MovementCalculation {
  target: u32,
  total_fuel_consumption: u32,
}

impl MovementCalculation {
  pub fn new(target: u32) -> Self {
    Self {
      target,
      total_fuel_consumption: 0,
    }
  }

  pub fn calculate_consumption(&mut self, positions: &Vec<u32>, cost_multiplication: bool) {
    for &pos in positions {
      let difference =  max(self.target, pos) - min(self.target, pos);
      if cost_multiplication {
        for i in 1..=difference {
          self.total_fuel_consumption += i
        }
      } else {
        self.total_fuel_consumption += difference;
      }

    }
  }
}

#[derive(Debug)]
struct Calc {
  positions: Vec<u32>,
  min_target: u32,
  max_target: u32,
  movement_calculations: Vec<MovementCalculation>
}

impl Calc {
  pub fn new(positions: Vec<u32>) -> Self {
    let &max_target = positions.iter().max().unwrap();
    let &min_target = positions.iter().min().unwrap();
    Self {
      positions,
      min_target,
      max_target,
      movement_calculations: Vec::new(),
    }
  }

  pub fn calculate_consumptions(&mut self, cost_multiplication: bool) {
    for i in self.min_target..=self.max_target {
      let mut movement_calc = MovementCalculation::new(i);
      movement_calc.calculate_consumption(&self.positions, cost_multiplication);
      self.movement_calculations.push(movement_calc);
    }
  }

  pub fn find_lowest_consumption_target(&self) -> u32 {
    return self.movement_calculations.iter().min_by_key(|calc| calc.total_fuel_consumption).unwrap().total_fuel_consumption;
  }

}



fn load_file_to_vector() -> Calc {
  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment7.txt"));
  let mut positions: Vec<u32> = Vec::new();
  match file_data {
    Ok(lines) => for result_line in lines {
      if let Ok(line) = result_line {
        positions = line
          .split(",")
          .map(|item| item.parse::<u32>().unwrap())
          .collect::<Vec<_>>();
      }
    },
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };

  return Calc::new(positions);
}
