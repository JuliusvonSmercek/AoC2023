use std::cmp::min;

pub fn convert(nums: &Vec<Vec<i64>>, source: i64) -> i64 {
  let mut mapped = source;
  for nums1 in nums {
    if 0 <= source - nums1[1] && source - nums1[1] < nums1[2] {
      mapped = source - nums1[1] + nums1[0];
    }
  }
  return mapped;
}

pub fn solve(input: &str) -> i64 {
  let lines: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
  let blocks: &[String] = &lines;
  let mut block_iter = blocks.iter();
  let seeds = block_iter
    .next()
    .unwrap()
    .get(("seeds: ".len())..)
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect::<Vec<i64>>();

  let nums: Vec<Vec<Vec<i64>>> = block_iter
    .map(|block| {
      block
        .split('\n')
        .skip(1)
        .map(|s| {
          s.split_whitespace()
            .map(|q| q.parse::<i64>().unwrap())
            .collect()
        })
        .collect()
    })
    .collect();

  let mut result = i64::MAX;
  for seed in seeds {
    let mut res = seed;
    for num1 in &nums {
      res = convert(num1, res);
    }
    result = min(result, res);
  }
  return result;
}
