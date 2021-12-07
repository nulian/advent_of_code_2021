use super::super::file_reader;
use std::path::Path;

pub fn run_assignment() {
  let mut pool = load_file_to_vector();
  println!("{:?}", pool);
  part1(&mut pool);
  let mut pool2 = load_file_to_vector();
  part2(&mut pool2);
}

fn part1(pool: &mut Pool) {
  pool.progress_time(80);
  println!("result part1: {}", pool.total_fish_count());
}

fn part2(pool: &mut Pool) {
  pool.progress_time(256);
  println!("result part2: {}", pool.total_fish_count());
}

#[derive(Debug)]
struct Fish {
  same_age_fishes: u64,
  internal_timer: u64,
  reset_time: u64,
}

impl Fish {
  pub fn new(initial_time: u64) -> Self {
    Self {
      internal_timer: initial_time,
      reset_time: 6,
      same_age_fishes: 0,
    }
  }

  pub fn add(&mut self, amount: u64) {
    // println!("same age fish: {}, amount: {}", self.same_age_fishes, amount);
    self.same_age_fishes += amount;
  }

  pub fn tick(&mut self) -> (bool, (u64, u64)) {
    if self.internal_timer > 6 {
      let move_to_next_fishes = self.same_age_fishes;
      self.same_age_fishes = 0;
      // println!("intt: {} move to next: {}", self.internal_timer, move_to_next_fishes);
      return (true, (self.internal_timer - 1, move_to_next_fishes));
    } else if self.internal_timer > 0 {
      // println!("intt: {} amount of fish: {}", self.internal_timer, self.same_age_fishes);
      self.internal_timer -= 1;
      return (false, (0, 0));
    } else {
      self.internal_timer = self.reset_time;
      return (false, (0, self.same_age_fishes));
    }
  }
}

#[derive(Debug)]
struct Pool {
  fishes: [Fish; 9],
}

impl Pool {
  pub fn new(fishes: Vec<u64>) -> Self {
    let mut fish_list: [Fish; 9] = [
      Fish::new(0),
      Fish::new(1),
      Fish::new(2),
      Fish::new(3),
      Fish::new(4),
      Fish::new(5),
      Fish::new(6),
      Fish::new(7),
      Fish::new(8),
    ];
    for fish in fishes {
      fish_list[fish as usize].add(1);
    }
    Self {
      fishes: fish_list,
    }
  }

  pub fn progress_time(&mut self, total_time: u64) {
    for _i in 0..total_time {
      let mut total_new_fishes: u64 = 0;
      let mut move_fishes: Vec<(u64, u64)> = Vec::new();
      for fish in self.fishes.iter_mut() {
        match fish.tick() {
          (false, (_, new_fishes)) => total_new_fishes += new_fishes,
          (true, event) => move_fishes.push(event),
        }
      }
      self.fishes[8].add(total_new_fishes);
      for (element, amount) in move_fishes {
        if element == 7 {
          self.fishes[7].add(amount);
        } else {
          let fish = self.fishes.iter_mut().find(|fish| fish.internal_timer == element).unwrap();
          fish.add(amount);
        }
      }
    }
  }

  pub fn total_fish_count(&self) -> u128 {
    let values = self.fishes.iter().map(|fish| fish.same_age_fishes as u128).collect::<Vec<u128>>();
    let result = values.iter().sum();
    return result;
  }
}

fn load_file_to_vector() -> Pool {
  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment6.txt"));
  let mut fishes: Vec<u64> = Vec::new();
  match file_data {
    Ok(lines) => for result_line in lines {
      if let Ok(line) = result_line {
        fishes = line
          .split(",")
          .map(|item| item.parse::<u64>().unwrap())
          .collect::<Vec<_>>();
      }
    },
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };

  return Pool::new(fishes);
}
