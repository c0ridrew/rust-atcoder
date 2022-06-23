// 問題文
// 3 つの数字 x,y,z をこの順に並べてできる 3 桁の整数を xyz と表すことにします。

// どの桁も 0 でない 3 桁の整数 abc が与えられるので、abc+bca+cab を求めてください。

// 制約
// abc は どの桁も 0 でない 3 桁の整数
// 入力
// 入力は以下の形式で標準入力から与えられる。

// abc
// 出力
// 答えを出力せよ。

use proconio::input;
fn main() {
  input! {
      abc: usize,
  }
  let abc_vec: Vec<_> = abc
    .to_string()
    .chars()
    .map(|d| d.to_digit(10).unwrap())
    .collect();

  let a = abc_vec[0];
  let b = abc_vec[1];
  let c = abc_vec[2];
  let results = (a * 100 + b * 10 + c) + (b * 100 + c * 10 + a) + (c * 100 + a * 10 + b);

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
