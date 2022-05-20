// https://atcoder.jp/contests/abc246/tasks/abc246_a

use std::collections::HashMap;

use proconio::input;

fn main() {
  input! {
    x_1: isize,
    y_1: isize,
    x_2: isize,
    y_2: isize,
    x_3: isize,
    y_3: isize,
  }

  let x_vec = vec![x_1, x_2, x_3];
  let y_vec = vec![y_1, y_2, y_3];

  let mut x_hashmap: HashMap<isize, usize> = HashMap::new();
  for v in x_vec {
    *x_hashmap.entry(v).or_default() += 1;
  }
  let mut y_hashmap: HashMap<isize, usize> = HashMap::new();
  for v in y_vec {
    *y_hashmap.entry(v).or_default() += 1;
  }

  let x_4 = x_hashmap.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().0;
  let y_4 = y_hashmap.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().0;

  println!("{0} {1}", x_4, y_4);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    main()
  }
}
