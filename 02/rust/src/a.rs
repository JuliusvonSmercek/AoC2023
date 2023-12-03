pub fn solve(lines: &[String]) -> i32 {
  let mut result = 0;
  for line in lines {
    let parts: Vec<&str> = line.split(": ").collect();
    assert!(2 == parts.len());

    let mut success = true;
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
      if 12 < red || 13 < green || 14 < blue {
        success = false;
      }
    }
    let game_num: i32 = parts[0].get(("Game ".len())..).unwrap().parse().unwrap();
    if success {
      result += game_num;
    }
  }
  return result;
}
