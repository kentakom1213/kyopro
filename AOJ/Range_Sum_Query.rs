// RSQ
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B&lang=ja

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

// BinaryIndexedTree
struct BIT {
    size: usize,
    arr: Vec<isize>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![0; n+1],
        }
    }

    fn build(src: &[isize]) -> Self {
        let size = src.len() + 1;
        let mut arr = vec![0; size];
        for i in 1..size-1 {
            let x = src[i - 1];
            arr[i] += x;
            let j = i + i.wrapping_neg();
            if j < size {
                arr[j] += arr[i];
            }
        }
        Self {
            size,
            arr,
        }
    }

    fn add(&mut self, mut i: usize, x: isize) {
        i += 1;
        while i <= self.size {
            self.arr[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix_sum(&self, mut i: usize) -> isize {
        let mut res = 0;
        while i != 0 {
            res += self.arr[i];
            i -= i & i.wrapping_neg();
        }
        res
    }
}

// solve
fn main() {
    let (n, q) = get!(usize, usize);

    let mut bit = BIT::new(n);

    for i in 0..q {
        let (com, x, y) = get!(usize, usize, usize);
        if com == 0 {
            let y = y as isize;
            bit.add(x-1, y);
        } else {
            let res = bit.prefix_sum(y) - bit.prefix_sum(x-1);
            println!("{}", res);            
        }
    }
}
