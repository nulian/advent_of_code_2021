use super::super::file_reader;
use regex::Regex;
use convert_case::{Case, Casing};
use std::path::Path;
use std::str::FromStr;
use strum_macros::EnumString;

pub fn run_assignment() {
  let mut map = load_file_to_vector();
  part1(&mut map);
  let mut map2 = load_file_to_vector();
  part2(&mut map2);
}

fn part1(map: &mut Map) {
  map.fill_map();
  map.calculate_fold_map();
  println!("{:?}", map);
  println!("result part1: {:?}", map.calculate_total());

}
//
fn part2(map: &mut Map) {
  map.fill_map();
  map.calculate_fold_map();
  for row in map.ground_map.iter() {
    for &i in row.iter() {
      if i == 1 {
        print!("#")
      } else {
        print!(".")
      }
    }
    print!("\n");
  }
}

#[derive(Debug, EnumString)]
enum Direction {
  X,
  Y,
}

#[derive(Debug)]
struct Point {
  x: u32,
  y: u32,
}

#[derive(Debug)]
struct Fold {
  direction: Direction,
  amount: u32,
}

#[derive(Debug)]
struct Map {
  points: Vec<Point>,
  ground_map: Vec<Vec<u8>>,
  folds: Vec<Fold>,
}

impl Map {
  pub fn new(points: Vec<Point>, folds: Vec<Fold>) -> Self {
    let (max_x, max_y) = Map::calculate_size(&points);
    Self {
      points,
      folds,
      ground_map: vec![vec![0; max_x as usize]; max_y as usize],
    }
  }

  fn fill_map(&mut self) {
    for point in self.points.iter() {
      self.ground_map[point.y as usize][point.x as usize] = 1;
    }
  }

  fn calculate_fold_map(&mut self) {
    for fold in self.folds.iter() {
      let mut folded_halve: Vec<Vec<u8>> = Vec::new();
      match fold.direction {
        Direction::Y =>{
          let max_x = self.ground_map.len();
          for i in (fold.amount as usize)..max_x {
            let mut row: Vec<u8> = Vec::new();
            for item in self.ground_map[i].iter() {
              row.push(item.clone());
            }
            folded_halve.push(row);
          }
          let row_size = self.ground_map[0].len();
          self.ground_map.resize(fold.amount as usize, vec![0; row_size]);
          folded_halve.reverse();
          Self::apply_folded_halve(&mut self.ground_map, &folded_halve);
        }
        Direction::X => {
          let row_size = self.ground_map[0].len();
          for map_row in self.ground_map.iter() {
            let mut row: Vec<u8> = Vec::new();
            for i in (fold.amount as usize)..row_size {
              row.push(map_row[i].clone());
            }
            row.reverse();
            folded_halve.push(row);
          }
          for map_row in self.ground_map.iter_mut() {
            map_row.resize(fold.amount as usize, 0);
          }
          Self::apply_folded_halve(&mut self.ground_map, &folded_halve);
        }
      }
    }
  }

  pub fn apply_folded_halve(ground_map: &mut Vec<Vec<u8>>, folded_halve: &Vec<Vec<u8>>) {
    for (i, row) in folded_halve.iter().enumerate() {
      for (j, item) in row.iter().enumerate() {
        if item.clone() == 1 {
          ground_map[i][j] = 1;
        }
      }

    }
  }

  fn calculate_total(&self) -> u32 {
    let mut total_count:u32 = 0;
    for row in self.ground_map.iter() {
      for &column in row.iter() {
        if column == 1 {
          total_count += 1;
        }
      }
    }
    return total_count;
  }

  fn calculate_size(points: &Vec<Point>) -> (u32, u32) {
    let max_x: u32 = points.iter().max_by_key(|&point| point.x).unwrap().x + 1;
    let max_y: u32 = points.iter().max_by_key(|&point| point.y).unwrap().y + 1;

    return (max_x, max_y);
  }
}


fn load_file_to_vector() -> Map {
  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment13.txt"));
  let mut positions: Vec<Point> = Vec::new();
  let mut folds: Vec<Fold> = Vec::new();
  let re = Regex::new(r"fold along (?P<key>\w+)=(?P<value>\d+)").unwrap();
  match file_data {
    Ok(lines) => for result_line in lines {
      if let Ok(line) = result_line {
        if line.contains(",") {
          let coord = line
            .split(",")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

          positions.push(Point { x: coord[0], y: coord[1] });
        } else if re.is_match(&line) {
          let captures = re.captures(&line).unwrap();
          let direction = Direction::from_str(&captures.name("key").unwrap().as_str().to_case(Case::Pascal).to_string()).unwrap();
          folds.push(Fold { direction, amount: captures.name("value").unwrap().as_str().parse::<u32>().unwrap() })
        }
      }
    },
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };

  return Map::new(positions, folds);
}
