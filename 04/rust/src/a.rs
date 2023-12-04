pub fn solve(lines: &[String]) -> i32 {
  let mut result = 0;
  for line in lines {
    let parts: Vec<&str> = line.split(": ").collect();

    let t1: Vec<&str> = parts[1].split(" | ").collect();
    let win_nums: Vec<i32> = t1[0]
      .split_whitespace()
      .map(|x| x.parse::<i32>().unwrap())
      .collect();
    let got_nums: Vec<i32> = t1[1]
      .split_whitespace()
      .map(|x| x.parse::<i32>().unwrap())
      .collect();

    let mut temp = 0;
    for w in win_nums {
      if got_nums.contains(&w) {
        if 0 == temp {
          temp = 1;
        } else {
          temp *= 2;
        }
      }
    }
    result += temp;
  }
  return result;
}
