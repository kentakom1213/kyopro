#![allow(dead_code)]

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// ## LCS
/// 最長共通部分列を得る
/// 計算量：O(NM)
fn LCS<T: std::cmp::PartialEq> (A: &[T], B: &[T]) -> usize {
    let (la, lb) = (A.len(), B.len());
    let mut dp = vec![vec![0; lb+1]; la+1];

    for (i, a) in A.iter().enumerate() {
        for (j, b) in B.iter().enumerate() {
            if a == b {
                chmax!(dp[i+1][j+1], dp[i][j] + 1);
            }
            chmax!(dp[i+1][j+1], dp[i+1][j]);
            chmax!(dp[i+1][j+1], dp[i][j+1]);
        }
    }

    dp[la][lb]
}

/// ## LCS with Vector
/// 最長共通部分列を得る
/// 計算量：O(NM)
fn LCS_with_Vec<T: std::cmp::PartialEq + Copy> (A: &[T], B: &[T]) -> Vec<T> {
    let (la, lb) = (A.len(), B.len());
    let mut dp = vec![vec![0; lb+1]; la+1];

    for (i, a) in A.iter().enumerate() {
        for (j, b) in B.iter().enumerate() {
            if a == b {
                chmax!(dp[i+1][j+1], dp[i][j] + 1);
            }
            chmax!(dp[i+1][j+1], dp[i+1][j]);
            chmax!(dp[i+1][j+1], dp[i][j+1]);
        }
    }

    let mut res: Vec<T> = vec![];
    let (mut cur, mut col) = (0, 0);
    'outer: for i in 0..la {
        for j in col..lb {
            if cur == dp[i][j] && dp[i][j] < dp[i+1][j+1] {
                res.push(A[i]);
                cur += 1;
                col = j + 1;
            }
            if cur == dp[la][lb] {  // LCSの長さに達したら終了
                break 'outer;
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_LCS_isize() {
        let a: Vec<isize> = vec![0, 1, 2, 3, 4, 5];
        let b: Vec<isize> = vec![-1, 1, 3, 5, 7, 9];

        assert_eq!(LCS(&a, &b), 3);
    }

    #[test]
    fn test_LCS_char() {
        let a: Vec<char> = "powell".chars().collect();
        let b: Vec<char> = "powershell".chars().collect();

        assert_eq!(LCS(&a, &b), 6);
    }

    #[test]
    fn test_LCS_with_Vec_usize() {
        let a: Vec<isize> = vec![0, 1, 2, 3, 4, 5];
        let b: Vec<isize> = vec![-1, 1, 3, 5, 7, 9];

        assert_eq!(LCS_with_Vec(&a, &b), vec![1, 3, 5]);
    }

    #[test]
    fn test_LCS_with_Vec_char() {
        let a: Vec<char> = "powell".chars().collect();
        let b: Vec<char> = "powershell".chars().collect();

        assert_eq!(LCS_with_Vec(&a, &b), vec!['p', 'o', 'w', 'e', 'l', 'l']);
    }
}
