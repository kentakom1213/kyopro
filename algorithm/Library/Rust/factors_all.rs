/// ## 高速約数列挙
/// `1 ~ N`までの数の約数を高速に列挙する
/// 計算量：`O(nloglogn)`
fn factors_all(n: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![]; n+1];
    for i in 1..=n {
        for j in 1.. {
            if i*j > n { break; }
            res[i*j].push(i);
        }
    }

    res
}
