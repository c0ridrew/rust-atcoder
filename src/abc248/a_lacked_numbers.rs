// 問題文
// 数字のみからなる、長さがちょうど 9 の文字列 S が与えられます。
// S には 0 から 9 までのうち、ちょうど 1 つの数字を除いた 9 種類の数字が一度ずつ登場します。

// S に登場しない唯一の数字を出力してください。

// 制約
// S は数字のみからなる長さ 9 の文字列である。
// S の文字はすべて相異なる。
// 入力
// 入力は以下の形式で標準入力から与えられる。

// S
// 出力
// S に登場しない唯一の数字を出力せよ。

// https://atcoder.jp/contests/abc248/tasks/abc248_a

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
