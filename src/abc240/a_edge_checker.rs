// https://atcoder.jp/contests/abc240/tasks/abc240_a

use proconio::input;
fn main() {
  input! {
      a: usize,
      b: usize,
  }
  let result = if a == (b + 1) || a == (b - 1) || (a + b) == 11 {
    "Yes"
  } else {
    "No"
  };
  println!("{}", result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    main()
  }
}
