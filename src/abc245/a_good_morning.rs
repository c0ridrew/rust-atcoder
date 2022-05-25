// 問題文
// ある日、高橋君は A 時 B 分ちょうどに、青木君は C 時 D 分 1 秒に起きました。
// 高橋君の起床時刻が青木君より早いならば Takahashi を、そうでないならば Aoki を出力してください。

// 制約
// 0≤A≤23
// 0≤B≤59
// 0≤C≤23
// 0≤D≤59
// 入力はすべて整数である。
// 入力
// 入力は以下の形式で標準入力から与えられる。

// A B C D
// 出力
// 高橋君の起床時刻が青木君より早いならば Takahashi を、そうでないならば Aoki を出力せよ。

// https://atcoder.jp/contests/abc245/tasks/abc245_a

use proconio::input;

fn main() {
  input! {
      a: usize,
      b: usize,
      c: usize,
      d: usize,
  }
  let results = if a < c || (a == c && b <= d) {
    "Takahashi"
  } else {
    "Aoki"
  };
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
