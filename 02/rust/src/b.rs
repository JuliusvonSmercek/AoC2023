use std::cmp::max;

pub fn solve(lines: &[String]) -> i32 {
  let mut result = 0;
  for line in lines {
    let parts: Vec<&str> = line.split(": ").collect();
    assert!(2 == parts.len());

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for item1 in parts[1].split("; ") {
      let mut red = 0;
      let mut green = 0;
      let mut blue = 0;
      for item2 in item1.split(", ") {
        let t4: Vec<&str> = item2.split(" ").collect();
        let num1: i32 = t4[0].parse().unwrap();
        if "red" == t4[1] {
          red += num1;
        } else if "green" == t4[1] {
          green += num1;
        } else if "blue" == t4[1] {
          blue += num1;
        }
      }
      max_red = max(max_red, red);
      max_green = max(max_green, green);
      max_blue = max(max_blue, blue);
    }
    result += max_red * max_green * max_blue;
  }
  return result;
}
