#[allow(dead_code)]

/// # RollingHash
#[derive(Debug)]
struct RollingHash {
    power: Vec<usize>,
    hash: Vec<usize>,
    base: usize,
}

impl RollingHash {
    const MOD: usize = (2 << 61) - 1;

    /// 初期化
    fn build(arr: &[usize], base: usize) -> Self {
        let size = arr.len();
        let mut power = vec![0; size + 1];
        let mut hash = vec![0; size + 1];

        // hashを初期化
        let mut v = 0;
        for i in 0..size {
            v = Self::madd( Self::mmul(v, base), arr[i]);
            hash[i+1] = v;
        }

        // powerを初期化
        let mut v = 1;
        for i in 0..size {
            v = Self::mmul(v, base);
            power[i+1] = v;
        }

        Self { power, hash, base }
    }

    /// 文字列から生成
    fn from_str(s: &str, base: usize) -> Self {
        let arr: Vec<usize> = s
            .chars()
            .map(Self::ord)
            .collect();
        
        Self::build(&arr, base)
    }

    /// `l..r`のハッシュを取得
    fn get(&self, l: usize, r: usize) -> usize {
        Self::msub(
            self.hash[r],
            Self::mmul(self.hash[l], self.power[r-l])
        )
    }

    /// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
    fn ord(c: char) -> usize {
        let a = 'A' as u32;
        let c = c as u32;
        (c - a) as usize
    }

    /// 足し算
    fn madd(mut a: usize, b: usize) -> usize {
        a += b;
        if a >= Self::MOD { a -= Self::MOD; }
        a
    }

    /// 引き算
    fn msub(mut a: usize, b: usize) -> usize {
        a += Self::MOD;
        a -= b;
        while a >= Self::MOD { a -= Self::MOD }
        a
    }

    /// 掛け算
    fn mmul(a: usize, b: usize) -> usize {
        let c: u128 = (a as u128) * (b as u128);
        (c % Self::MOD as u128) as usize
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pattern_match() {
        let base = 20021213;

        let target = RollingHash::from_str("momomosumomomomomonouchi", base);
        let ptn1 = RollingHash::from_str("sumomo", base);
        let ptn2 = RollingHash::from_str("momo", base);
        let (tlen, p1len, p2len) = (24, 6, 4);

        // "sumomo"を検索
        let mut res1 = vec![];
        for i in 0..tlen-p1len {
            if target.get(i, i + p1len) == ptn1.get(0, p1len) {
                res1.push(i);
            }
        }

        assert_eq!(res1, vec![6]);

        // "momo"を検索
        let mut res2 = vec![];
        for i in 0..tlen-p2len {
            if target.get(i, i + p2len) == ptn2.get(0, p2len) {
                res2.push(i);
            }
        }

        assert_eq!(res2, vec![0, 2, 8, 10, 12, 14]);
    }
}
