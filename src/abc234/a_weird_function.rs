// 問題文
// 関数 f を f(x)=x
// 2
//  +2x+3 と定義します。
// 整数 t が入力されるので、 f(f(f(t)+t)+f(f(t))) を求めてください。
// ただし、答えは 2×10
// 9
//   以下の整数であることが保証されます。

// 制約
// t は 0 以上 10 以下の整数である
// 入力
// 入力は以下の形式で標準入力から与えられる。

// t
// 出力
// 答えを整数として出力せよ。

use proconio::input;
fn main() {
  input! {
      t: f64,
  }
  let f = |x: f64| x.powf(2 as f64) + 2 as f64 * x + 3 as f64;
  let result = f(f(f(t) + t) + f(f(t)));

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
