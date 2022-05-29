// 問題文
// 高橋君の家には、高橋君、高橋君の父、高橋君の母の 3 人が住んでおり、全員が毎晩風呂で髪を洗います。
// 風呂には、高橋君の父、高橋君の母、高橋君の順に入り、それぞれシャンプーを A,B,C ミリリットル使います。

// 今朝の時点で、ボトルには V ミリリットルのシャンプーが残っていました。このまま補充しない時、初めてシャンプーが不足するのは誰が使おうとした時ですか？

// 制約
// 1≤V,A,B,C≤10^5
// 入力に含まれる値は全て整数である
// 入力
// 入力は以下の形式で標準入力から与えられる。

// V A B C
// 出力
// 初めてシャンプーが不足するのが、高橋君の父が使おうとしたときならば F、高橋君の母が使おうとしたときならば M、高橋君が使おうとしたときならば T を出力せよ。

// https://atcoder.jp/contests/abc243/tasks/abc243_a

use proconio::input;
fn main() {
  input! {
      v: usize,
      a: usize,
      b: usize,
      c: usize
  }
  let remain = v % (a + b + c);
  let results = if remain < a {
    "F"
  } else if remain < a + b {
    "M"
  } else {
    "T"
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
