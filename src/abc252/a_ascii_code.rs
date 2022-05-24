// A - ASCII code  /
// 実行時間制限: 2 sec / メモリ制限: 1024 MB

// 配点 : 100 点

// 問題文
// 英小文字 a, b, …, z の ASCII 文字コードはこの順に 97,98,…,122 です。

// 97 以上 122 以下の整数 N が与えられるので、ASCII 文字コードが N であるような英小文字を出力してください。

// 制約
// N は 97 以上 122 以下の整数
// 入力
// 入力は以下の形式で標準入力から与えられる。

// N
// 出力
// 答えを出力せよ。

// https://atcoder.jp/contests/abc252/tasks/abc252_a

use proconio::input;

fn main() {
  input! {
    n: usize,
  }
  let alphabets = (b'a'..=b'z') // Start as u8
    .map(|c| c as char) // Convert all to chars
    .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
    .collect::<Vec<_>>();
  let results = alphabets[n - 97];

  println!("{}", results);
}

// 別解
// use proconio::input;
// fn main() {
//     input! {
//         n: u8,
//     }
//     println!("{}", n as char);
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    main()
  }
}
