// 問題文
// 100 以上の整数 N が与えられます。N の下 2 桁を出力してください。

// ただし、N の下 2 桁とは十の位と一の位をこの順に並べたものを言います。

// 制約
// 100≤N≤999
// N は整数である。
// 入力
// 入力は以下の形式で標準入力から与えられる。

// N
// 出力
// 答えを出力せよ。

// https://atcoder.jp/contests/abc254/tasks/abc254_a

use proconio::input;
fn main() {
  input! {
      n: usize,
  }
  let n_string_array: Vec<_> = n
    .to_string()
    .chars()
    .map(|d| d.to_digit(10).unwrap().to_string())
    .collect();

  let first_digit = &n_string_array[1];
  let second_digit = &n_string_array[2];
  let results = format!("{}{}", first_digit, second_digit);
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
