// 問題文
// 縦 H 行、横 W 列のマス目があり、このうち上から i 個目、左から j 個目のマスを (i,j) と呼びます。
// このとき、マス (R,C) に辺で隣接するマスの個数を求めてください。

// ただし、ある 2 つのマス (a,b),(c,d) が辺で隣接するとは、 ∣a−c∣+∣b−d∣=1 (∣x∣ を x の絶対値とする) であることを言います。

// 制約
// 入力は全て整数
// 1≤R≤H≤10
// 1≤C≤W≤10
// 入力
// 入力は以下の形式で標準入力から与えられる。

// H W
// R C
// 出力
// 答えを整数として出力せよ。

// https://atcoder.jp/contests/abc250/tasks/abc250_a

use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
    r: usize,
    c: usize,
  }

  let h_count = if h == 1 {
    0
  } else if r == 1 || r == h {
    1
  } else {
    2
  };

  let w_count = if w == 1 {
    0
  } else if c == 1 || w == c {
    1
  } else {
    2
  };
  let result = h_count + w_count;

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
