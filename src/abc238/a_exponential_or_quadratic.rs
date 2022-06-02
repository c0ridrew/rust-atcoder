// 問題文
// 2
// n
//  >n
// 2
//   ですか？

// 制約
// n は 1 以上 10
// 9
//   以下の整数
// 入力
// 入力は以下の形式で標準入力から与えられる。

// n
// 出力
// 2
// n
//  >n
// 2
//   なら Yes を、そうでないなら No を出力せよ。

// https://atcoder.jp/contests/abc238/tasks/abc238_a

use proconio::input;
fn main() {
  input! {
    n: f64,
  }
  let result = if (2 as f64).powf(n as f64) > n.powf(2 as f64) {
    "Yes"
  } else {
    "No"
  };
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
