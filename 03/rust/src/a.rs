fn is_digit(c: char) -> bool {
  return '0' <= c && c <= '9';
}

fn is_symbol(c: char) -> bool {
  return !is_digit(c) && '.' != c;
}

pub fn solve(lines: &[String]) -> u32 {
  let mut result = 0;
  for i in 0..lines.len() {
    let mut j = 0;
    while j < lines[i].len() {
      while j < lines[i].len() && !is_digit(lines[i].chars().nth(j).unwrap()) {
        j += 1;
      }
      let mut number = 0;
      let start_j = j as i32;
      while j < lines[i].len() && is_digit(lines[i].chars().nth(j).unwrap()) {
        let res1 = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap();
        number = number * 10 + res1;
        j += 1;
      }
      let mut condition = false;
      for i1 in (i as i32 - 1)..(i as i32 + 2) {
        for j1 in (start_j as i32 - 1)..(j as i32 + 1) {
          if 0 <= i1 && i1 < lines.len() as i32 && 0 <= j1 && j1 < lines[i1 as usize].len() as i32 {
            condition |= is_symbol(lines[i1 as usize].chars().nth(j1 as usize).unwrap());
          }
        }
      }
      if condition {
        result += number;
      }
    }
  }
  return result;
}
