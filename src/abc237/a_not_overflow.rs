// 問題文
// 整数 N が与えられます。 N が −2
// 31
//   以上かつ 2
// 31
//   未満ならば Yes を、そうでないならば No を出力してください。

// 制約
// −2
// 63
//  ≤N<2
// 63
// N は整数である。
// 入力
// 入力は以下の形式で標準入力から与えられる。

// N
// 出力
// N が −2
// 31
//   以上かつ 2
// 31
//   未満ならば Yes を、そうでないならば No を出力せよ。

use proconio::input;

fn main() {
  input! {
      n: f64,
  }
  let result = if (-2 as f64).powf(31 as f64) <= n && n < (2 as f64).powf(31 as f64) {
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
