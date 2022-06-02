// 問題文
// 1 桁の数字が表示される画面と、ボタンからなる機械があります。

// 画面に数字 k が表示されているとき、ボタンを 1 回押すと画面の数字が a
// k
// ​
//   に変わります。

// 0 が表示されている状態からボタンを 3 回押すと、画面には何が表示されますか？

// 制約
// 0≤a
// i
// ​
//  ≤9
// 入力は全て整数である
// 入力
// 入力は以下の形式で標準入力から与えられる。

// a
// 0
// ​
//   a
// 1
// ​
//   … a
// 9
// ​

// https://atcoder.jp/contests/abc241/tasks/abc241_a

use proconio::input;
fn main() {
  input! {
      a0: usize,
      a1: usize,
      a2: usize,
      a3: usize,
      a4: usize,
      a5: usize,
      a6: usize,
      a7: usize,
      a8: usize,
      a9: usize,
  }
  let a_list = [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9];
  let result = a_list[a_list[a_list[0]]];
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
