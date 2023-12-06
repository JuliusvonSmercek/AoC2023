mod a;
mod b;

use std::fs;

fn load_file(file: &str) -> String {
  let contents: String =
    fs::read_to_string(file).expect(&format!("Error: reading file '{}'", file));
  return contents;
}

fn main() {
  for file_a in vec!["../data/sample.txt", "../data/input.txt"] {
    println!("a with '{file_a}'\t: {}", a::solve(&load_file(file_a)));
  }

  for file_b in vec!["../data/sample.txt", "../data/input.txt"] {
    println!("b with '{file_b}'\t: {}", b::solve(&load_file(file_b)));
  }
}
