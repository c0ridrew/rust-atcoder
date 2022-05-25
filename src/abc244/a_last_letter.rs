// A - Last Letter  /
// 実行時間制限: 2 sec / メモリ制限: 1024 MB

// 配点 : 100 点

// 問題文
// 英小文字からなる長さ N の文字列 S が与えられます。S の末尾の文字を出力してください。

// 制約
// N は整数
// 1≤N≤1000
// S は英小文字からなる長さ N の文字列
// 入力
// 入力は以下の形式で標準入力から与えられる。

// N
// S
// 出力
// S の末尾の文字を出力せよ。

use proconio::input;
fn main() {
  input! {
      n: usize,
      s: String,
  }
  let s_array: Vec<char> = s.chars().collect();
  let results = s_array.last().clone().unwrap();
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
