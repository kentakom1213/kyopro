//               E - Notebook              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc273/tasks/abc273_e
// ----------------------------------------

macro_rules! get_vals {
    ($($t:ty), *) => {
        {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let mut iter = s.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
}

struct Node {
    val: isize,
    par: usize,
    child: Vec<usize>,
}

impl Node {
    fn new(val: isize, par: usize) -> Node {
        Node {
            val,
            par,
            child: Vec::new(),
        }
    }
}

fn main() {
    let (q,) = get_vals!(usize);

    let mut tree = vec![ Node::new(-1, 0) ];
    let mut v = 0;

    for _ in 0..q {
        let (cmd,) = get_vals!(String);
        match cmd.as_str() {
            "ADD" => {
                let x = get_vals!(usize);
            },
            "DELETE" => (),
            "SAVE" => {
                let x = get_vals!(usize);
            },
            "LOAD" => {
                let x = get_vals!(usize);
            },
        }
    }
}
