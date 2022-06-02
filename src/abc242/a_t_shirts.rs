// 問題文
// あるプログラミングコンテストでは、以下のルールに従って参加者に T シャツをプレゼントします。

// 上位 A 位までの参加者は、必ず T シャツが貰える。
// 加えて、上位 A+1 位から B 位までの参加者のうち C 人が一様ランダムに選ばれ、選ばれた参加者は T シャツを貰える。
// コンテストには 1000 人が参加し、全ての参加者が相異なる順位を取りました。
// このコンテストの参加者であるいろはちゃんは、X 位を取りました。
// このとき、いろはちゃんが T シャツを貰える確率を求めてください。

// 制約
// 入力はすべて整数
// 1≤A<B≤1000
// 1≤C≤B−A
// 1≤X≤1000
// 入力
// 入力は以下の形式で標準入力から与えられる。

// A B C X
// 出力
// 答えを出力せよ。 なお、想定解との絶対誤差または相対誤差が 10
// −6
//   以下であれば、正解として扱われる。

use proconio::input;

fn main() {
  input! {
      a: String,
  }
  println!("{}", a);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    main()
  }
}
