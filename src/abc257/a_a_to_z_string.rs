
// 問題文
// A を N 個、B を N 個、…、Z を N 個この順に繋げて得られる文字列の先頭から X 番目の文字を求めてください。

// 制約
// 1≤N≤100
// 1≤X≤N×26
// 入力は全て整数
// 入力
// 入力は以下の形式で標準入力から与えられる。

// N X
// 出力
// 答えを出力せよ。

use proconio::input;
fn main() {
    input! {
        n: usize,
        x: usize,
    }
  let alphabets = (b'A'..=b'Z') // Start as u8
    .map(|c|c as char) // Convert all to chars
    .collect::<Vec<_>>();

  let mut vec = Vec::new();
  for v in alphabets.iter() {
    for _ in 0..n {
      vec.push(v);
    }
  }

  let result = &vec[x-1];
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
