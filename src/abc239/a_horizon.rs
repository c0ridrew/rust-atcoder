use proconio::input;
fn main() {
  input! {
      h: usize,
  }
  let result = ((h * (12800000 + h)) as f64).sqrt();
  println!("{}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    main()
  }
}
