// 配点 : 100 点

// 問題文
// サンタさんに手紙を出したい高橋くんは、 X 円切手が 1 枚だけ貼られた封筒を用意しました。
// サンタさんに手紙を届けるためには、貼られている切手の総額が Y 円以上である必要があります。
// 高橋くんは、この封筒に 10 円切手を何枚か貼り足すことで、貼られている切手の総額を Y 円以上にしたいです。
// 高橋くんはこの封筒に、最小で何枚の 10 円切手を貼り足す必要がありますか?

// 制約
// X,Y は整数
// 1≤X,Y≤1000

use proconio::input;
fn main() {
    input! {
        x: isize,
        y: isize,
    }
    let rest = y - x;
    let result = if rest <= 0 {
        0
    } else if rest % 10 != 0 {
        rest / 10 + 1
    } else {
        rest / 10
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
