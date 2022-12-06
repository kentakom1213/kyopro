// # 参考
// - https://makandat.wordpress.com/2022/02/08/rust-%E3%81%AE%E5%8B%89%E5%BC%B7-%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E4%B8%80%E8%A6%A7/
// - https://qiita.com/benki/items/70ad2ee44cff9efde778

use std::path;
use colored::*;

const ROOT_DIR: &str = "..";

fn main() {
    let root = path::PathBuf::from(ROOT_DIR);
    link_rust_file::traverse_dir(&root);

    println!("{}", "Complete!".green().bold());
}
