// 問題文
// 縦 A 行、横 B 列のマスからなるタイルを縦 N 行、横 N 列に並べてできた、縦 (A×N) 行、横 (B×N) 列のマス目 X があります。
// 1≤i,j≤N について、上から i 行目、左から j 列目のタイルをタイル (i,j) とします。

// X の各マスは以下のように塗られています。

// 各タイルは白いタイルまたは黒いタイルである。
// 白いタイルのすべてのマスは白で塗られ、黒いタイルのすべてのマスは黒で塗られている。
// タイル (1,1) は白いタイルである。
// 辺で隣接する 2 つのタイルは異なる色のタイルである。ただし、タイル (a,b) とタイル (c,d) が辺で隣接するとは、∣a−c∣+∣b−d∣=1 ( ∣x∣ を x の絶対値とする)であることを言う。
// マス目 X を出力の形式に従って出力してください。

// 制約
// 1≤N,A,B≤10
// 入力は全て整数
// 入力
// 入力は以下の形式で標準入力から与えられる。

// N A B
// 出力
// 次の条件をみたす (A×N) 個の文字列 S
// 1
// ​
//  ,…,S
// A×N
// ​
//   を改行区切りで出力せよ。

// S
// 1
// ​
//  ,…,S
// A×N
// ​
//   はそれぞれ長さ (B×N) の . または # からなる文字列である。
// 各 i,j (1≤i≤A×N,1≤j≤B×N) に対し、マス目 X の上から i 行目かつ左から j 列目のマスが白で塗られているならば S
// i
// ​
//   の j 文字目は .であり、黒く塗られているならば # である。

// https://atcoder.jp/contests/abc250/tasks/abc250_b

// TODO: solve it later
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: usize,
    b: usize,
  }

  let mut stack = vec![vec![0; n * b]; n * a];

  let height = n * a;
  let width = n * b;

  for i in 1..width {
    stack.push(".".repeat(a));
    stack.push("#".repeat(n));
  }

  // let m = a.abs_diff(c) + b.abs_diff()=1

  for i in 1..width {
    println!("{}", i);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    main()
  }
}
