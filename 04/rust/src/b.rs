pub fn solve(lines: &[String]) -> i32 {
  let mut result = 0;

  let mut dp: Vec<i32> = vec![1; lines.len() + 1];
  for line in lines {
    let parts: Vec<&str> = line.split(": ").collect();
    let game_num_str: Vec<&str> = parts[0].split_whitespace().collect();
    let game_num: usize = game_num_str[1].parse().unwrap();

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
        temp += 1;
      }
    }

    for i in 0..temp {
      dp[game_num + i + 1] += dp[game_num];
    }

    result += dp[game_num];
  }

  return result;
}
