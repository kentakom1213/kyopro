// # 参考
// - https://makandat.wordpress.com/2022/02/08/rust-%E3%81%AE%E5%8B%89%E5%BC%B7-%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E4%B8%80%E8%A6%A7/
// - https://qiita.com/benki/items/70ad2ee44cff9efde778

use std::path;
use colored::*;
use link_rust_files::{
    traverse_dir,
    make_lib,
};

const ROOT_DIR: &str = "..";

fn main() {
    let root = path::PathBuf::from(ROOT_DIR);
    // traverse_dir(&root);
    let mut lib: Vec<(usize, String)> = vec![];
    make_lib(&root, &mut lib, 0);

    // 表示
    println!("{}", "kyopuro".green());
    for (d, s) in &lib {
        if s.ends_with(".rs") {
            println!("{} ├─ {}", " │ ".repeat(*d), s.blue());
        } else {
            println!("{} ├─ {}", " │ ".repeat(*d), s.green());
        }
    }

    println!("{}", "Complete!".green().bold());
}
