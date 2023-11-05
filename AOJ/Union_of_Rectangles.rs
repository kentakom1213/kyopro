//           Union of Rectangles
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_4_A&lang=ja
// ----------------------------------------

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

/// # 座標圧縮
#[derive(Debug)]
pub struct Compression<'a, T> {
    pub size: usize,
    pub sorted_array: Vec<&'a T>,
}
impl<'a, T: Ord> Compression<'a, T> {
    /// スライス`array`で配列を初期化する
    pub fn new(array: &'a [T]) -> Self {
        let mut comp: Vec<&T> = array.iter().collect();
        comp.sort();
        comp.dedup();
        Self {
            size: comp.len(),
            sorted_array: comp,
        }
    }
    /// 圧縮後の`val`の番号を返す
    pub fn idx(&self, val: &T) -> Option<usize> {
        let idx = self.sorted_array.binary_search(&val);
        if let Ok(idx) = idx {
            Some(idx)
        } else {
            None
        }
    }
    /// 圧縮前の要素`idx`を返す
    pub fn val(&self, idx: usize) -> Option<&T> {
        if let Some(&val) = self.sorted_array.get(idx) {
            Some(val)
        } else {
            None
        }
    }
}

fn main() {
    let n = get!(usize);
    let rects: Vec<(isize, isize, isize, isize)> =
        (0..n).map(|_| get!(isize, isize, isize, isize)).collect();

    let mut xs = vec![];
    let mut ys = vec![];
    for &(x1, y1, x2, y2) in &rects {
        xs.push(x1);
        xs.push(x2);
        ys.push(y1);
        ys.push(y2);
    }

    let cx = Compression::new(&xs);
    let cy = Compression::new(&ys);

    let mut imos = vec![vec![0; cx.size + 1]; cy.size + 1];

    for &(x1, y1, x2, y2) in &rects {
        let x1 = cx.idx(&x1).unwrap();
        let y1 = cy.idx(&y1).unwrap();
        let x2 = cx.idx(&x2).unwrap();
        let y2 = cy.idx(&y2).unwrap();

        imos[y1][x1] += 1;
        imos[y2][x1] -= 1;
        imos[y1][x2] -= 1;
        imos[y2][x2] += 1;
    }

    // 累積和
    for i in 0..=cy.size {
        for j in 1..=cx.size {
            imos[i][j] += imos[i][j - 1];
        }
    }

    for j in 0..=cx.size {
        for i in 1..=cy.size {
            imos[i][j] += imos[i - 1][j];
        }
    }

    if cfg!(debug_assertions) {
        eprintln!("cx = {:?}", &cx);
        eprintln!("cy = {:?}", &cy);
        for row in &imos {
            eprintln!("{:?}", row);
        }
    }

    let mut ans = 0;

    for i in 0..cy.size - 1 {
        for j in 0..cx.size - 1 {
            if imos[i][j] > 0 {
                if let &[&y1, &y2] = &cy.sorted_array[i..i + 2] {
                    if let &[&x1, &x2] = &cx.sorted_array[j..j + 2] {
                        ans += (x2 - x1) * (y2 - y1);
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
