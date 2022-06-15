use proconio::input;

fn main() {
  input! {
      r: usize,
      c: usize,
      a_1_1: usize,
      a_1_2: usize,
      a_2_1: usize,
      a_2_2: usize,
  }
  let a_list = [[a_1_1, a_1_2], [a_2_1, a_2_2]];
  let results = a_list[r - 1][c - 1];

  println!("{}", results);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    main()
  }
}
