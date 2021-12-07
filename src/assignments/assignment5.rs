use super::super::file_reader;
use regex::Regex;
use std::path::Path;

pub fn run_assignment() {
  let mut map = load_file_to_vector();
  part1(&mut map);
  let mut map2 = load_file_to_vector();
  part2(&mut map2);
}

fn part1(map: &mut Map) {
  map.plot_vents(false);
  println!("result part1: {}", map.amount_of_dangerous_points());

}

fn part2(map: &mut Map) {
  map.plot_vents(true);
  println!("result part2: {}", map.amount_of_dangerous_points());

}
#[derive(Debug)]
struct Point {
  x: u32,
  y: u32,
}

#[derive(Debug)]
struct Coord {
  start: Point,
  end: Point,
  diagonally: bool,
  max_x: u32,
  max_y: u32,
}

impl Coord {
  pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
    Self {
      start: Point { x: x1, y: y1 },
      end: Point { x: x2, y: y2 },
      diagonally: !(x1 == x2 || y1 == y2),
      max_x: if x1 > x2 { x1 } else { x2 } + 1,
      max_y: if y1 > y2 { y1 } else { y2 } + 1,
    }
  }
}

#[derive(Debug)]
struct Map {
  coords: Vec<Coord>,
  ground_map: Vec<Vec<u8>>,
}

impl Map {
  pub fn new(coords: Vec<Coord>) -> Self {
    let (max_x, max_y) = Map::calculate_size(&coords);
    Self {
      coords,
      ground_map: vec![vec![0; max_y as usize]; max_x as usize],
    }
  }

  pub fn amount_of_dangerous_points(&self) -> u32 {
    let mut dangerous_sections: u32  = 0;
    for vert in self.ground_map.iter() {
      for &hor in vert {
        if hor >= 2 {
          dangerous_sections += 1;
        }
      }
    }
    return dangerous_sections;
  }

  pub fn plot_vents(&mut self, count_diagonally: bool) {
    for coord in self.coords.iter() {
      if count_diagonally && coord.diagonally {
       let range_x = Map::maybe_reverse_range(coord.start.x, coord.end.x, coord.start.x > coord.end.x);
       let mut range_y = Map::maybe_reverse_range(coord.start.y, coord.end.y, coord.start.y > coord.end.y);
       for x in range_x {
         let y = range_y.next().unwrap();
         self.ground_map[x as usize][y as usize] += 1;
       }
      } else if !coord.diagonally {
        for x in Map::maybe_reverse_range(coord.start.x, coord.end.x, coord.start.x > coord.end.x) {
          for y in Map::maybe_reverse_range(coord.start.y, coord.end.y, coord.start.y > coord.end.y) {
            self.ground_map[x as usize][y as usize] += 1;
          }
        }
      }
    }
  }

  fn maybe_reverse_range(init: u32, end: u32, reverse: bool) -> Box<dyn Iterator<Item=u32>> {
    if reverse {
      Box::new((end..=init).rev())
    } else {
      Box::new(init..=end)
    }
  }

  fn calculate_size(coords: &Vec<Coord>) -> (u32, u32) {
    let max_x: u32 = coords.iter().max_by_key(|&coord| coord.max_x).unwrap().max_x;
    let max_y: u32 = coords.iter().max_by_key(|&coord| coord.max_y).unwrap().max_y;

    return (max_x, max_y);
  }
}

fn load_file_to_vector() -> Map {
  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment5.txt"));
  let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
  let mut coords: Vec<Coord> = Vec::new();
  match file_data {
    Ok(lines) => for result_line in lines {
      if let Ok(line) = result_line {
        if let Some(captures) = re.captures(&line) {
          coords.push(Coord::new(captures.name("x1").unwrap().as_str().parse::<u32>().unwrap(),           captures.name("y1").unwrap().as_str().parse::<u32>().unwrap(),          captures.name("x2").unwrap().as_str().parse::<u32>().unwrap(),          captures.name("y2").unwrap().as_str().parse::<u32>().unwrap()));

        }
      }
    },
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };

  return Map::new(coords);
}
