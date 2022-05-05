// 英大文字と英小文字からなる文字列のうち、以下の条件を全て満たすものを素晴らしい文字列ということとします。

// 英大文字が文字列の中に現れる。
// 英小文字が文字列の中に現れる。
// 全ての文字が相異なる。
// 例えば、AtCoder や Aa は素晴らしい文字列ですが、atcoder や Perfect は素晴らしい文字列ではありません。

// 文字列 S が与えられるので、S が素晴らしい文字列か判定してください。

// 制約
// 1≤∣S∣≤100
// S は英大文字と英小文字からなる文字列である。
// 入力
// 入力は以下の形式で標準入力から与えられる。

// S
// 出力
// S が素晴らしい文字列ならば Yes を、そうでないならば No を出力せよ。

// https://atcoder.jp/contests/abc249/tasks/abc249_b

use proconio::input;

fn main() {
  input! {
    S: String,
  }

  let includes_uppercase = S.chars().any(|c| c.is_uppercase());
  let includes_lowercase = S.chars().any(|c| c.is_lowercase());
  let is_duplicated = _contains_duplicate(S);

  let mut result = "";
  if includes_lowercase && includes_uppercase && !is_duplicated {
    result = "Yes"
  } else {
    result = "No"
  }

  println!("{}", result);
}

fn _contains_duplicate(s: String) -> bool {
  let x = s.chars().collect::<Vec<char>>();
  let mut y = x.clone();
  y.sort();
  y.dedup();
  return x.len() != y.len();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    main()
  }
}
