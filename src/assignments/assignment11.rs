use super::super::file_reader;
use std::path::Path;

pub fn run_assignment() {
  let mut map = load_file_to_vector();
  part1(&mut map);
  let mut map2 = load_file_to_vector();
  part2(&mut map2);
}
//
fn part1(map: &mut Map) {

  for _ in 0..100 {
    map.execute_step();
  }
  println!("total flashed: {}", map.total_flashed);


}

fn part2(map: &mut Map) {
  let mut step_flash_all = 0;
    for i in 0..1000 {
    if map.execute_step() {
      step_flash_all = i + 1;
      break;
    }
  }
  println!("all flashed: {}", step_flash_all);
}

#[derive(Debug)]
struct Point {
  x: usize,
  y: usize,
}

#[derive(Debug)]
struct Octopus {
  location: Point,
  energy_level: usize,
  flashed: bool,
}

impl Octopus {
  pub fn new(location: Point, energy_level: usize) -> Self {
    Self {
      location,
      energy_level,
      flashed: false
    }
  }

  pub fn increase_energy(&mut self) -> bool {
    self.energy_level += 1;
    if self.energy_level > 9 && !self.flashed {
      return true;
    }
    return false;
  }

  pub fn reset_if_flashed(&mut self) {
    if self.flashed {
      self.energy_level = 0;
      self.flashed = false;
    }
  }

  pub fn affected_fields(&self) -> Option<Vec<Point>> {
    if self.energy_level > 9 {
      let mut update_points: Vec<Point> = Vec::new();
      for y in -1..=1 {
        for x in -1..=1 {
          if !(y == 0 && y == x) {
            let new_x = self.location.x as i32 + x;
            let new_y = self.location.y as i32 + y;

            if new_x >= 0 && new_x < 10 && new_y >= 0 && new_y < 10 {
              update_points.push(Point {x: new_x as usize, y: new_y as usize});
            }

          }
        }
      }
      if update_points.len() > 0 {
        return Some(update_points);
      }
      return None;
    } else {
      return None;
    }
  }
}

#[derive(Debug)]
struct Map {
  octopus_map: Vec<Vec<Octopus>>,
  total_flashed: u32,
}

impl Map {
  pub fn new(octopus_map: Vec<Vec<Octopus>>) -> Self {
    Self {
      octopus_map,
      total_flashed: 0,
    }
  }

  pub fn execute_step(&mut self) -> bool {

    let mut flash = true;
    let mut step_flash = 0;
    for octopus_row in self.octopus_map.iter_mut() {
      for octopus in octopus_row.iter_mut() {
        let result = octopus.increase_energy();
        if result {
          flash = true;
        }
      }
    }


    while flash {
      flash = false;
      let mut collected_updates: Vec<Point> = Vec::new();
      for (_y, octopus_row) in self.octopus_map.iter_mut().enumerate() {
        for (_x, octopus) in octopus_row.iter_mut().enumerate() {
          if octopus.energy_level > 9 && !octopus.flashed {
            if let Some(mut affected_fields) = octopus.affected_fields() {
              collected_updates.append(&mut affected_fields);
              octopus.flashed = true;
              step_flash += 1;
              self.total_flashed += 1;
            }
          }
        }
      }

      for update in collected_updates {
        let result = self.octopus_map[update.y][update.x].increase_energy();
        if result {
          flash = true;
        }
      }
    }

    for octopus_row in self.octopus_map.iter_mut() {
      for octopus in octopus_row.iter_mut() {
        octopus.reset_if_flashed();
      }
    }
    // println!();
    // for row in self.octopus_map.iter() {
    //   for i in row.iter() {
    //     print!("{}", i.energy_level);
    //   }
    //   print!("\n");
    // }

    return step_flash == 100;
  }
}


fn load_file_to_vector() -> Map {
  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment11.txt"));
  let mut octopus_map: Vec<Vec<Octopus>> = Vec::new();
  match file_data {
    Ok(lines) => for (y, result_line) in lines.enumerate() {
      if let Ok(line) = result_line {
        let mut row: Vec<Octopus> = Vec::new();
        for (x, char) in line.chars().enumerate() {
          row.push(Octopus::new(Point {x, y}, char.to_digit(10).unwrap() as usize))
        }
        octopus_map.push(row);
      }
    },
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };

  return Map::new(octopus_map);
}
