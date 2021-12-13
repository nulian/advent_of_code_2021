use super::super::file_reader;
use std::collections::HashSet;
use std::path::Path;
pub fn run_assignment() {
  let mut data = load_file_to_vector();
  println!("{:?}", data);
  part1(&mut data);
  let mut pool2 = load_file_to_vector();
  part2(&mut pool2);
}

fn part1(data: &mut Data) {
  println!(
    "result part1: {:?}",
    data.calculate_total_unique_output_values()
  );
}
fn part2(data: &mut Data) {
  data.calculate_signal_values();
  data.calculate_output_results();
  println!("{:?}", data);
  println!("{:?}", data.calculate_total_output_value());
  // println!("result part2: {:?}", calc.find_lowest_consumption_target());
}
#[derive(Debug)]
struct Signal {
  segments: HashSet<char>,
  number_value: Option<u8>,
  total_lit: u8,
}

impl Signal {
  pub fn new(segments: HashSet<char>) -> Self {
    let size = segments.len() as u8;
    Self {
      segments,
      number_value: Signal::get_value_by_key(size),
      total_lit: size,
    }
  }

  pub fn get_value_by_key(key: u8) -> Option<u8> {
    match key {
      2 => Some(1),
      3 => Some(7),
      4 => Some(4),
      7 => Some(8),
      _ => None,
    }
  }
}

#[derive(Debug)]
struct Display {
  all_signal_patterns: Vec<Signal>,
  output_values: Vec<usize>,
  output_results: Vec<u32>,
}

impl Display {
  pub fn new(all_signal_patterns: Vec<Signal>, output_values: Vec<usize>) -> Self {
    Self {
      all_signal_patterns,
      output_values,
      output_results: Vec::new()
    }
  }

  pub fn calculate_remaining_signal_number(&mut self) {
    self.calculate_5_lit_numbers();
    self.calculate_6_lit_numbers();
  }

  pub fn calculate_5_lit_numbers(&mut self) {
    let one_signal = self.fetch_signal_chars(1).unwrap();
    let four_signal = self.fetch_signal_chars(4).unwrap();

    if let Some(three_signal) = self
      .all_signal_patterns
      .iter_mut()
      .find(|signal| signal.total_lit == 5 && signal.segments.is_superset(&one_signal))
    {
      three_signal.number_value = Some(3);
    }

    if let Some(five_signal) = self.all_signal_patterns.iter_mut().find(|signal| {
      let remaining_four_signal_diff = four_signal
        .difference(&signal.segments)
        .map(|char| char.clone())
        .collect::<String>();
      let remaining_one_signal_diff = one_signal
        .difference(&signal.segments)
        .map(|char| char.clone())
        .collect::<String>();
      return signal.total_lit == 5
        && remaining_four_signal_diff.len() > 0
        && remaining_four_signal_diff == remaining_one_signal_diff;
    }) {
      five_signal.number_value = Some(5);
    }

    if let Some(two_signal) = self
      .all_signal_patterns
      .iter_mut()
      .find(|signal| signal.total_lit == 5 && signal.number_value == None)
    {
      two_signal.number_value = Some(2);
    }
  }

  pub fn calculate_6_lit_numbers(&mut self) {
    let one_signal = self.fetch_signal_chars(1).unwrap();
    let five_signal = self.fetch_signal_chars(5).unwrap();
    let total_lit: u8 = 6;

    if let Some(six_signal) = self.all_signal_patterns.iter_mut().find(|signal| signal.total_lit == total_lit && !signal.segments.is_superset(&one_signal)) {
      six_signal.number_value = Some(6);
    }
    let nine_segments = five_signal.union(&one_signal).map(|char| char.clone()).collect::<HashSet<_>>();

    if let Some(nine_signal) = self.all_signal_patterns.iter_mut().find(|signal| signal.total_lit == total_lit && signal.number_value == None && signal.segments.eq(&nine_segments)) {
      nine_signal.number_value = Some(9)
    }

    if let Some(zero_signal) = self.all_signal_patterns.iter_mut().find(|signal| signal.total_lit == total_lit && signal.number_value == None) {
      zero_signal.number_value = Some(0)
    }
  }

  fn fetch_signal_chars(&self, number_value: u8) -> Option<HashSet<char>> {
    if let Some(signal) = self
      .all_signal_patterns
      .iter()
      .find(|signal| signal.number_value == Some(number_value))
    {
      return Some(signal.segments.iter().map(|&x| x.clone()).collect());
    } else {
      return None;
    }
  }

  pub fn get_amount_of_unique_calculated_output_values(&self) -> u8 {
    let mut total_unique_output_values: u8 = 0;
    for value in self.output_values.iter() {
      let signal = &self.all_signal_patterns[*value];
      if None != signal.number_value {
        total_unique_output_values += 1;
      }
    }
    return total_unique_output_values;
  }

  pub fn calculate_output_results(&mut self) {
    for value in self.output_values.iter() {
      let signal = &self.all_signal_patterns[*value];
      if let Some(number) = signal.number_value {
        self.output_results.push(number as u32)
      }
    }
  }
}

#[derive(Debug)]
struct Data {
  displays: Vec<Display>,
}

impl Data {
  pub fn calculate_total_unique_output_values(&self) -> u32 {
    let mut total_unique_output_values: u32 = 0;
    for display in &self.displays {
      total_unique_output_values += display.get_amount_of_unique_calculated_output_values() as u32;
    }
    return total_unique_output_values;
  }

  pub fn calculate_total_output_value(&self) -> u32 {
    let mut total_output_value: u32 = 0;
    for display in &self.displays {
      println!("display: {:?}", display);
      println!("result: {:?}", display.output_results.iter().map(|i| String::from(i.to_string())).collect::<String>().parse::<u32>().unwrap());
      total_output_value += display.output_results.iter().map(|i| String::from(i.to_string())).collect::<String>().parse::<u32>().unwrap();
    }
    return total_output_value;
  }

  pub fn calculate_signal_values(&mut self) {
    for display in self.displays.iter_mut() {
      display.calculate_remaining_signal_number();
    }
  }

  pub fn calculate_output_results(&mut self) {
    for display in self.displays.iter_mut() {
      display.calculate_output_results();
    }
  }
}

fn load_file_to_vector() -> Data {
  let file_data = file_reader::read_lines(Path::new("./src/assignments/inputs/assignment8.txt"));
  let mut all_displays: Vec<Display> = Vec::new();
  match file_data {
    Ok(lines) => {
      for result_line in lines {
        if let Ok(line) = result_line {
          let parsed_line = line.split(" | ").collect::<Vec<_>>();
          let signals = parsed_line[0];
          let display_signals = signals
            .split_whitespace()
            .map(|signal| Signal::new(signal.chars().collect::<HashSet<_>>()))
            .collect::<Vec<_>>();
          let output = parsed_line[1];
          let mut output_signals = Vec::new();
          for signal in output.split_whitespace() {
            let current_output_signal = signal.chars().collect::<HashSet<_>>();
            let idx = display_signals
              .iter()
              .position(|signal| signal.segments == current_output_signal)
              .unwrap();
            output_signals.push(idx);
          }
          all_displays.push(Display::new(display_signals, output_signals));
        }
      }
    }
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };

  return Data {
    displays: all_displays,
  };
}
