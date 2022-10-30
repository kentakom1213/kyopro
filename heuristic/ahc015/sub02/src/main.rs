#![allow(non_snake_case)]

use proconio::input;
use rand::distributions::{Distribution, Uniform};

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

// データの大きさ
static SIZE: usize = 100;
static DIR: [char; 4] = ['L', 'R', 'F', 'B'];
static SEED: [u8; 32] = [7; 32];

// フィールド
struct Field {
    n: usize,  // 現在の個数
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


// solve
fn main() {
    // 生成器の初期化
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(SEED);
    let die = Uniform::from(0..4);

    let f = get!(usize;;);
    let mut field = Field {
        n: 0,
        cells: vec![vec![0; 10]; 10],
    };

    for i in 0..SIZE {
        // 入力の受け取り
        let candy_type = f[i];
        let s = get!(usize);

        // ランダムに方向を決める
        let rand_d = die.sample(&mut rng);
        println!("{}", DIR[rand_d]);
        
        // 置かれた位置にキャンディーを置く
        field.set(s, candy_type);

        match rand_d {
            0 => field.to_L(),
            1 => field.to_R(),
            2 => field.to_F(),
            3 => field.to_B(),
            _ => unreachable!(),
        }
    }
}

