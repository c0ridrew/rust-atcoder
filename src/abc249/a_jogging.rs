// 実行時間制限: 2 sec / メモリ制限: 1024 MB

// 配点 : 100 点

// 問題文
// 高橋君と青木君はジョギングをすることにしました。
// 高橋君は「A 秒間秒速 B メートルで歩き、C 秒間休む」ことを繰り返します。
// 青木君は「D 秒間秒速 E メートルで歩き、F 秒間休む」ことを繰り返します。
// 二人が同時にジョギングを始めてから X 秒後、高橋君と青木君のうちどちらが長い距離を進んでいますか？

// 制約
// 1≤A,B,C,D,E,F,X≤100
// 入力は全て整数
// 入力
// 入力は以下の形式で標準入力から与えられる。

// A B C D E F X
// 出力
// 二人が同時にジョギングを始めてから X 秒後時点で、高橋君の方が青木君よりも長い距離を進んでいるならば Takahashi、青木君の方が高橋君よりも長い距離を進んでいるならば Aoki、二人が同じ距離を進んでいるならば Draw と出力せよ。

// https://atcoder.jp/contests/abc249/tasks/abc249_a

use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    f: usize,
    x: usize,
  }

  let takahashi = b / (a + c) * x;
  let aoki = e / (d + f) * x;
  let result = if takahashi > aoki {
    "takahashi"
  } else if aoki > takahashi {
    "aoki"
  } else {
    "draw"
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
