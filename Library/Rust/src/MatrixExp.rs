#[allow(dead_code)]
use rand::prelude::*;

// 定数
const MOD: usize = 998244353;

/* 行列累乗 */
const DIM: usize = 4;
type Vec = [usize; DIM];
type Matrix = [[usize; DIM]; DIM];

trait MatrixExp {
    fn e() -> Self;
    fn dot(&self, other: Self) -> Self;
    fn pow(&self, e: usize) -> Self;
    fn apply(&self, vec: Vec) -> Vec;
}

impl MatrixExp for Matrix {
    /// ## e
    /// 単位行列を返す
    fn e() -> Self {
        let mut res = [[0; DIM]; DIM];
        for i in 0..DIM {
            res[i][i] = 1;
        }
        res
    }

    /// ## apply
    /// ベクトル`x`と行列`A`について、`Ax`を返す
    fn apply(&self, vec: Vec) -> Vec {
        let mut res = [0; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                res[i] += self[i][j] * vec[j] % MOD;
                res[i] %= MOD;
            }
        }
        res
    }

    /// ## pow
    /// 行列の累乗を返す（繰り返し2乗法）
    fn pow(&self, mut e: usize) -> Self {
        let mut res = Self::e();
        let mut tmp = self.clone();
        while e > 0 {
            if e & 1 == 1 {
                res = tmp.dot(res);
            }
            tmp = tmp.dot(tmp);
            e >>= 1;
        }
        res
    }

    /// ## dot
    /// 行列のドット積
    fn dot(&self, other: Self) -> Self {
        let mut res = [[0; DIM]; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                for k in 0..DIM {
                    res[i][j] += self[i][k] * other[k][j] % MOD;
                    res[i][j] %= MOD;
                }
            }
        }
        res
    }
}


#[cfg(test)]
mod test {
    use super::*;
    const REPEAT_TIME: usize = 100;

    /// ランダムな値で埋められたDIMxDIM行列を生成する
    fn gen_random_matrix(rng: &mut ThreadRng) -> [[usize; DIM]; DIM] {
        let mut res = [[0; DIM]; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                res[i][j] = rng.gen::<usize>() % MOD;
            }
        }
        res
    }

    /// ランダムな1bitの値で埋められたDIM次元行列を生成する
    fn gen_random_vector(rng: &mut ThreadRng) -> [usize; DIM] {
        let mut res = [0; DIM];
        for i in 0..DIM {
            res[i] = rng.gen::<bool>() as usize;
        }
        res
    }

    /// ## test_dot
    /// 行列積の正当性の検証
    /// - [乱択アルゴリズム紹介(行列乗算の検査&多項式等価性の検査)](https://tech.preferred.jp/ja/blog/matrix-multiplication-and-polynomial-identity/)
    #[test]
    fn test_dot() {
        let mut rng = rand::thread_rng();

        // ランダムな行列を生成
        let A = gen_random_matrix(&mut rng);
        let B = gen_random_matrix(&mut rng);

        for _ in 0..REPEAT_TIME {
            // ランダムなベクトルを生成
            let v = gen_random_vector(&mut rng);

            // left = (A @ B) @ v
            let left = A.dot(B).apply(v);
            // right = A @ (B @ v)
            let right = A.apply(B.apply(v));

            assert_eq!(left, right);
        }
    }

    /// ## test_pow
    /// 行列累乗の正当性の検証
    #[test]
    fn test_pow() {
        // テトラナッチ数
        let tetra = [
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
            [1, 1, 1, 1],
        ];

        // 初期値
        let init = [0, 0, 0, 1];
        
        // T_{35}を求める
        let T_35 = tetra.pow(35).apply(init)[0];

        assert_eq!(T_35, 747044834);
    }
}
