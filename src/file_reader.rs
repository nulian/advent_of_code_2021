use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where
    P: AsRef<Path>,
{
  let file = File::open(filename)?;
  return Ok(io::BufReader::new(file).lines());
}

pub fn mut_read_lines<P>(filename: P) -> io::BufReader<File>
  where
    P: AsRef<Path>,
{
  let file = File::open(filename);
  match file {
    Ok(file) => {
      return io::BufReader::new(file);
    }
    Err(error) => panic!("Problem opening the file: {:?}", error),
  }
}
