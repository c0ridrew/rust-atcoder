// 問題文
// 英小文字からなる文字列 S が与えられます。

// S の先頭から a 文字目と b 文字目を入れ替えて得られる文字列を出力してください。

// 制約
// S は英小文字からなる文字列
// S の長さ ∣S∣ は、 2≤∣S∣≤10 を満たす
// 1≤a<b≤∣S∣
// a,b は整数
// 入力
// 入力は以下の形式で標準入力から与えられる。

// S
// a b
// 出力
// 答えを出力せよ。

use proconio::input;

fn main() {
  input! {
      s: String,
      a: usize,
      b: usize,
  }

  let mut s_array: Vec<char> = s.chars().collect();
  s_array.swap(a - 1, b - 1);
  let results: String = s_array.iter().collect();
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
