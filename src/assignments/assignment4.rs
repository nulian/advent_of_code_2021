use super::super::file_reader;
use regex::Regex;
use std::io::BufRead;
use std::path::Path;

pub fn run_assignment() {
  let mut game = load_file_to_vector();
  part1(&mut game);
  part2(&mut game);
}

fn part1(game: &mut Game) {
  game.play_game(WinTime::First);
  let total: i32 = game.win_board_number();
  println!("result of assignment1: {}", total);
}

fn part2(game: &mut Game) {
  game.play_game(WinTime::Last);
  let total: i32 = game.win_board_number();
  println!("result of assignment1: {}", total);
}

const TRUE_LINE: [bool; 5] = [true; 5];

#[derive(PartialEq)]
enum WinTime {
  First,
  Last,
}

#[derive(Debug)]
struct Game {
  number_draw_list: Vec<u8>,
  boards: Vec<Board>,
  previous_drawn_number: u8,
  last_win_board: usize,
  total_won_boards: usize,
}

impl Game {
  fn new() -> Self {
    Self {
      number_draw_list: Vec::new(),
      boards: Vec::new(),
      previous_drawn_number: 0,
      last_win_board: 0,
      total_won_boards: 0,
    }
  }

  pub fn win_board_number(&self) -> i32 {
    let board = self.boards.get(self.last_win_board).unwrap();
    return board.sum_of_unmarked_fields() * self.previous_drawn_number as i32;
  }

  pub fn play_game(&mut self, win_time: WinTime) {
    let total_boards = self.boards.len();
    'outer: for number in &self.number_draw_list {
      self.previous_drawn_number = *number;
      for (i, board) in self.boards.iter_mut().enumerate() {
        board.mark_ball(*number);
        if board.bingo() == (false, true) {
          self.last_win_board = i;
          self.total_won_boards += 1;
          match win_time {
            WinTime::First => break 'outer,
            WinTime::Last => {
              if self.total_won_boards == total_boards {
                break 'outer;
              }
            }
          }
        }
      }
    }
  }
}

#[derive(Debug)]
struct Board {
  board: [[u8; 5]; 5],
  lookup: Vec<u8>,
  marked: [[bool; 5]; 5],
  bingo: bool,
}

impl Board {
  fn new(board: [[u8; 5]; 5], lookup: Vec<u8>) -> Self {
    Self {
      board,
      marked: [[false; 5]; 5],
      lookup,
      bingo: false,
    }
  }

  pub fn bingo(&mut self) -> (bool, bool) {
    if self.bingo {
      return (true, true)
    }

    for &row in self.marked.iter() {
      if row == TRUE_LINE {
        self.bingo = true;
        return (false, true);
      }
    }

    for i in 0..4 {
      if self.fetch_column(i) == TRUE_LINE {
        self.bingo = true;
        return (false, true);
      }
    }

    return (false, false);
  }

  pub fn mark_ball(&mut self, number: u8) {
    if Some(&number) == self.lookup.iter().find(|&&elem| elem == number) {
      for i in 0..=4 {
        for j in 0..=4 {
          if self.board[i][j] == number {
            self.marked[i][j] = true;
          }
        }
      }
    }
  }

  pub fn sum_of_unmarked_fields(&self) -> i32 {
    let mut total = 0;
    for i in 0..=4 {
      for j in 0..=4 {
        if !self.marked[i][j] {
          total += self.board[i][j] as i32;
        }
      }
    }
    return total;
  }

  fn fetch_column(&self, column_index: usize) -> Vec<bool> {
    return self
      .marked
      .iter()
      .map(|s| s.iter().nth(column_index).unwrap().clone())
      .collect::<Vec<_>>();
  }
}

fn load_file_to_vector() -> Game {
  let mut game = Game::new();
  let mut lines =
    file_reader::mut_read_lines(Path::new("./src/assignments/inputs/assignment4.txt")).lines();
  let re = Regex::new(r"(\d+)").unwrap();

  if let Some(line) = lines.next() {
    game.number_draw_list = line
      .unwrap()
      .split(",")
      .map(|item| item.parse::<u8>().unwrap())
      .collect();
  }
  while let Some(_newline) = lines.next() {
    let mut array: [[u8; 5]; 5] = [[0; 5]; 5];
    let mut lookup: Vec<u8> = Vec::new();
    for i in 0..=4 {
      if let Some(Ok(text_line)) = lines.next() {
        println!("{}", text_line);
        let test = re.captures_iter(&text_line).enumerate();
        for (j, caps) in test {
          let item = caps[0].parse::<u8>().unwrap();
          array[i][j] = item;
          lookup.push(item);
        }
      }
    }
    game.boards.push(Board::new(array, lookup));
  }
  return game;
}
