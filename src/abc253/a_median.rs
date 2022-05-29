// 問題文
// 整数 a,b,c が与えられます。b がこれらの整数の中央値であるかどうか判定してください。

// 制約
// 1≤a,b,c≤100
// 入力は全て整数
// 入力
// 入力は以下の形式で標準入力から与えられる。

// a b c
// 出力
// b が与えられた整数の中央値であるならば Yes、そうでないならば No と出力せよ。

// https://atcoder.jp/contests/abc253/tasks/abc253_a

use proconio::input;
fn main() {
  input! {
      a: usize,
      b: usize,
      c: usize,
  }
  let mut order = vec![a, b, c];
  order.sort();
  let result = if order[1] == b { "Yes" } else { "No" };
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
