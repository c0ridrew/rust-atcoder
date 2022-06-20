// 問題文
// N が与えられます。2
// N
//   を出力してください。

// 制約
// 0≤N≤30
// N は整数である
// 入力
// 入力は以下の形式で標準入力から与えられる。

// N
// 出力
// 答えを出力せよ。

// https://atcoder.jp/contests/abc256/tasks/abc256_a

use proconio::input;
fn main() {
  input! {
      n: f64,
  }
  let result = (2 as f64).powf(n);
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
