// 問題文
// A 匹のスライムがいます。

// すぬけくんが 1 回叫ぶたびに、スライムは K 倍に増殖します。

// スライムが B 匹以上になるには、すぬけくんは最小で何回叫ぶ必要があるでしょうか？

// 制約
// 1≤A≤B≤10
// 9

// 2≤K≤10
// 9
// 入力は全て整数
// 入力
// 入力は以下の形式で標準入力から与えられる。

// A B K
// 出力
// 答えを出力せよ。

// https://atcoder.jp/contests/abc248/tasks/abc248_b

use proconio::input;

fn main() {
    input! {
        mut a: i32,
        b: i32,
        k: i32
    }

    let mut results = 0;

    while a < b {
        a = a * k;
        results += 1;
    }

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
