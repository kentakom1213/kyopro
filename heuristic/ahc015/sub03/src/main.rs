#![allow(non_snake_case)]

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

// 定数
static SIZE: usize = 100;  // データの大きさ
static DIR: [char; 4] = ['L', 'R', 'F', 'B'];  // 方向
static SEED: [u8; 32] = [7; 32];  // 乱数シード

// [RustでUnionFind](https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb)
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    // UnionFindを新規作成
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    // 根を求める
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);  // 経路圧縮
        self.par[x]
    }

    // 同一の集合に所属するか判定
    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // 要素を結合
    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        // 要素数が大きい方を子にすることで、高さを均等に保つ
        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }

}

// フィールド
#[derive(Clone)]
struct Field {
    cells: Vec<Vec<usize>>,  // 現在のフィールドの状況
}

// フィールドのメソッド
impl Field {
    // 指定された位置にキャンディをセット
    fn set(&mut self, pos: usize, candy_type: usize) {
        let mut cnt = 0;
        'outer: for r in 0..10 {
            for c in 0..10 {
                if self.cells[r][c] == 0 {
                    cnt += 1;
                }
                if cnt == pos {
                    self.cells[r][c] = candy_type;
                    break 'outer;
                }
            }
        }
    }

    // 全ての行についてキャンディを左端に寄せる
    fn to_L(&mut self) {
        for r in 0..10 {
            let mut cnt = 0;
            for c in 0..10 {
                if self.cells[r][c] != 0 {
                    self.cells[r][cnt] = self.cells[r][c];
                    if cnt != c {
                        self.cells[r][c] = 0;
                    }
                    cnt += 1;
                }
            }
        }
    }

    // 全ての行についてキャンディを右端に寄せる
    fn to_R(&mut self) {
        for r in 0..10 {
            let mut cnt = 9;
            for c in (0..10).rev() {
                if self.cells[r][c] != 0 {
                    self.cells[r][cnt] = self.cells[r][c];
                    if cnt != c {
                        self.cells[r][c] = 0;
                    }
                    cnt -= 1;
                    if cnt <= 0 { break; }
                }
            }
        }
    }

    // 全ての列についてキャンディを上に寄せる
    fn to_F(&mut self) {
        for c in 0..10 {
            let mut cnt = 0;
            for r in 0..10 {
                if self.cells[r][c] != 0 {
                    self.cells[cnt][c] = self.cells[r][c];
                    if cnt != r {
                        self.cells[r][c] = 0;
                    }
                    cnt += 1;
                }
            }
        }
    }

    // 全ての列についてキャンディを下に寄せる
    fn to_B(&mut self) {
        for c in 0..10 {
            let mut cnt = 9;
            for r in (0..10).rev() {
                if self.cells[r][c] != 0 {
                    self.cells[cnt][c] = self.cells[r][c];
                    if cnt != r {
                        self.cells[r][c] = 0;
                    }
                    cnt -= 1;
                    if cnt <= 0 { break; }
                }
            }
        }
    }

    // 表示する
    fn print(&self) {
        for row in &self.cells {
            row.iter().for_each(|v| eprint!("{} ", v));
            eprintln!();
        }
        eprintln!();
    }
}

// 連結成分のリストを求める
fn get_connections(cells: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut uf = UnionFind::new(100);  // unionfind
    for i in 0..100 {
        let (r, c) = (i/10, i%10);

        // 左方向
        if c > 0 {
            if cells[r][c] == cells[r][c-1] {
                uf.unite(i-1, i);
            }
        }
        // 右方向
        if c < 9 {
            if cells[r][c] == cells[r][c+1] {
                uf.unite(i, i+1);
            }
        }
        // 上方向
        if r > 0 {
            if cells[r][c] == cells[r-1][c] {
                uf.unite(i-10, i);
            }
        }
        // 下方向
        if r < 9 {
            if cells[r][c] == cells[r+1][c] {
                uf.unite(i, i+10);
            }
        }
    }

    // unionfindの各連結成分の大きさを求める
    let mut cnts = [0; 100];
    for i in 0..100 {
        let (r, c) = (i/10, i%10);
        let root = uf.root(i);
        if cells[r][c] != 0 {
            cnts[root] += 1;
        }
    }

    let mut res = vec![];
    for i in 0..100 {
        if cnts[i] > 0 {
            res.push(cnts[i]);
        }
    }

    res
}


// solve
fn main() {

    let f = get!(usize;;);
    let mut field = Field {
        cells: vec![vec![0; 10]; 10],
    };

    for i in 0..SIZE {
        // 入力の受け取り
        let candy_type = f[i];
        let s = get!(usize);

        // 置かれた位置にキャンディーを置く
        field.set(s, candy_type);

        // スコアの分母
        let d2 = {
            let mut candy_cnts = [0; 3];
            for &v in &f[..i] {
                candy_cnts[v-1] += 1;
            }
            candy_cnts.iter().map(|v| (v*v) as f64).fold(0.0, |acc, v| acc+v)
        };

        // 左に傾けたときのスコア
        let score_l = {
            let mut field2 = field.clone();
            field2.to_L();
    
            // スコアの分子
            let n2 = get_connections(&field2.cells).iter().map(|v| (v*v) as f64).fold(0.0, |acc, v| acc+v);
            // スコア
            (1000000.0 * n2 / d2).round()
        };

        // 右に傾けたときのスコア
        let score_r = {
            let mut field2 = field.clone();
            field2.to_R();
    
            // スコアの分子
            let n2 = get_connections(&field2.cells).iter().map(|v| (v*v) as f64).fold(0.0, |acc, v| acc+v);
            // スコア
            (1000000.0 * n2 / d2).round()
        };

        // 上に傾けたときのスコア
        let score_f = {
            let mut field2 = field.clone();
            field2.to_F();
    
            // スコアの分子
            let n2 = get_connections(&field2.cells).iter().map(|v| (v*v) as f64).fold(0.0, |acc, v| acc+v);
            // スコア
            (1000000.0 * n2 / d2).round()
        };

        // 下に傾けたときのスコア
        let score_b = {
            let mut field2 = field.clone();
            field2.to_B();
    
            // スコアの分子
            let n2 = get_connections(&field2.cells).iter().map(|v| (v*v) as f64).fold(0.0, |acc, v| acc+v);
            // スコア
            (1000000.0 * n2 / d2).round()
        };

        eprintln!("{} {} {} {}", score_l, score_r, score_f, score_b);

        // 最大値を求める
        let max = score_l
                    .max(score_r)
                    .max(score_f)
                    .max(score_b);
        
        match max {
            score_l => {
                field.to_L();
                println!("L");
            },
            score_r => {
                field.to_R();
                println!("R");
            },
            score_f => {
                field.to_F();
                println!("F");
            },
            score_b => {
                field.to_B();
                println!("D");
            },
        }
    }
}

