// 問題文
// 横一列に 4 つのマスが並んでいます。

// 各文字が 0 または 1 である長さ 4 の文字列 S が与えられます。
// S の i 文字目が 1 であるとき、左から i 番目のマスには 1 人の人がおり、
// S の i 文字目が 0 であるとき、左から i 番目のマスには人がいません。

// 全ての人が一斉に、1 つ右隣のマスへ移動します。この移動により、もともと右端のマスにいた人は消えます。

// 移動後の各マスに人がいるかどうかを、S と同様のルールで文字列として出力してください。

// 制約
// S は 0, 1 のみからなる長さ 4 の文字列
// 入力
// 入力は以下の形式で標準入力から与えられる。

// S
// 出力
// 長さ 4 の文字列であって、移動後、左から i 番目のマスに人がいるならば i 文字目が 1、いないならば 0 であるようなものを出力せよ。

// https://atcoder.jp/contests/abc247/tasks/abc247_a

use proconio::input;

fn main() {
  input! {
      S: String,
  }

  let sum = 45;
  let a = S
    .chars()
    .fold(0, |acc, c| c.to_digit(10).unwrap_or(0) + acc);

  let result = sum - a;
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
