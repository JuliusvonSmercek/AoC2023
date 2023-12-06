// runtime: ~63s

pub fn inv_convert(nums: &Vec<Vec<i64>>, mapped: i64) -> i64 {
  let mut source = mapped;
  for nums1 in nums {
    if 0 <= mapped - nums1[0] && mapped - nums1[0] < nums1[2] {
      source = mapped - nums1[0] + nums1[1];
      break;
    }
  }
  return source;
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

  let mut counter: i64 = 0;
  loop {
    if counter % 1000000 == 0 {
      dbg!(counter / 1000000);
    }
    let mut inv_val = counter;
    for iq in 0..7 {
      inv_val = inv_convert(&nums[6 - iq], inv_val);
    }
    for i in 0..(seeds.len() / 2) {
      let a = seeds[2 * i];
      let b = seeds[2 * i] + seeds[2 * i + 1];
      let item = inv_val;
      if a <= item && item < b {
        return counter;
      }
    }
    counter += 1;
  }
}
