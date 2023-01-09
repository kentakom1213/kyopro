#[allow(dead_code)]

/// ## factoring
/// - 素因数分解し、`(素因数,指数)`のペアを返す
fn factoring(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in 2.. {
        if i*i > n {
            break;
        }
        let mut cnt = 0;
        while n % i == 0 {
            n /= i;
            cnt += 1;
        }
        if cnt >= 1 {
            res.push((i, cnt));
        }
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factoring() {
        assert_eq!(
            factoring(1024),
            vec![(2, 10)]
        );

        assert_eq!(
            factoring(123456789),
            vec![(3, 2), (3607, 1), (3803, 1)]
        );

        assert_eq!(
            factoring(20021213),
            vec![(20021213, 1)]
        );

        assert_eq!(
            factoring(1234567891234567),
            vec![(47, 1), (167, 1), (167953, 1), (936511, 1)]
        );
    }
}
