use super::super::file_reader;
use regex::Regex;
use std::path::Path;

pub fn run_assignment() {
  let mut map = load_file_to_vector();
  println!("{:?}", map)
  // part1(&mut map);
  // let mut map2 = load_file_to_vector();
  // part2(&mut map2);
}

// fn part1(map: &mut Map) {
//   map.plot_vents(false);
//   println!("result part1: {}", map.amount_of_dangerous_points());
//
// }
//
// fn part2(map: &mut Map) {
//   map.plot_vents(true);
//   println!("result part2: {}", map.amount_of_dangerous_points());
//
// }
#[derive(Debug)]
struct Point {
  x: u32,
  y: u32,
}

#[derive(Debug)]
struct Map {
  points: Vec<Point>,
  ground_map: Vec<Vec<u8>>,
}

impl Map {
  pub fn new(points: Vec<Point>) -> Self {
    let (max_x, max_y) = Map::calculate_size(&points);
    Self {
      points,
      ground_map: vec![vec![0; max_y as usize]; max_x as usize],
    }
  }

  fn calculate_size(points: &Vec<Point>) -> (u32, u32) {
    let max_x: u32 = points.iter().max_by_key(|&point| point.x).unwrap().x;
    let max_y: u32 = points.iter().max_by_key(|&point| point.y).unwrap().y;

    return (max_x, max_y);
  }
}


fn load_file_to_vector() -> Map {
  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment13.txt"));
  let mut positions: Vec<Point> = Vec::new();
  match file_data {
    Ok(lines) => for result_line in lines {
      if let Ok(line) = result_line {
        if line.contains(",") {
          let coord = line
            .split(",")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

          positions.push(Point {x: coord[0], y: coord[1]});
        }
      }
    },
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };

  return Map::new(positions);
}
