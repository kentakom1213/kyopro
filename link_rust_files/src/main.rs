use std::path;
use colored::*;
use link_rust_files::{
    make_tree,
    display_tree,
    write_lib,
};

const ROOT_DIR: &str = "..";
const LIB_PATH: &str = "../lib.rs";

fn main() {
    let root = path::PathBuf::from(ROOT_DIR);
    let lib_path = path::PathBuf::from(LIB_PATH);

    // `lib.rs`のベースとなるリストを取得
    let mut lib: Vec<(usize, String)> = vec![];
    make_tree(&root, &mut lib, 0);

    // tree形式で表示を行う
    display_tree(&lib);
    
    // ファイルに書き出し
    write_lib(&lib, &lib_path);

    println!("\n{}", "Complete!".green().bold());
}
