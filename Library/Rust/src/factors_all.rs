#![allow(dead_code)]

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


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_factors_all() {
        let facs = factors_all(10);
        assert_eq!(
            facs,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 3],
                vec![1, 2, 4],
                vec![1, 5],
                vec![1, 2, 3, 6],
                vec![1, 7],
                vec![1, 2, 4, 8],
                vec![1, 3, 9],
                vec![1, 2, 5, 10],
            ]
        );
    }
}
