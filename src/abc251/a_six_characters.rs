// 問題文
// 英小文字からなる文字列 S が与えられます。 S の長さは 1 以上かつ 3 以下です。

// S を繰り返して得られる文字列であって、長さが 6 のものを出力してください。

// 本問題の制約下で、そのような文字列はただ一つ存在することが示せます。

// 制約
// S は英小文字からなる長さ 1 以上 3 以下の文字列
// 入力
// 入力は以下の形式で標準入力から与えられる。

// S
// 出力
// 答えとなる長さ 6 の文字列を出力せよ。

// https://atcoder.jp/contests/abc251/tasks/abc251_a

use proconio::input;

fn main() {
  input! {
    s: String,
  }
  let s_count = s.chars().count();
  let result = if s_count == 1 {
    s.repeat(6)
  } else if s_count == 2 {
    s.repeat(3)
  } else {
    s.repeat(2)
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
