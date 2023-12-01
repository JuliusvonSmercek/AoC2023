fn process_line(line: &String, smaller: bool) -> i32 {
  let mut index: usize = 0;
  let mut result: Option<i32> = None;
  let numbers: Vec<(i32, &str)> = vec![
    (0, "0"),
    (1, "1"),
    (2, "2"),
    (3, "3"),
    (4, "4"),
    (5, "5"),
    (6, "6"),
    (7, "7"),
    (8, "8"),
    (9, "9"),
  ];
  for (value, repr) in numbers {
    let pos = if smaller {
      line.find(repr)
    } else {
      line.rfind(repr)
    };
    if pos == None {
      continue;
    }
    if result.is_none()
      || (if smaller {
        pos.unwrap() < index
      } else {
        pos.unwrap() > index
      })
    {
      index = pos.unwrap();
      result = value.into();
    }
  }
  return result.expect("Error");
}

pub fn solve(lines: &[String]) -> i32 {
  let mut result = 0;
  for line in lines {
    result += process_line(line, true) * 10 + process_line(line, false);
  }
  return result;
}
