use colored::*;
use link_files::{
    make_tree,
    // display_tree,
    write_lib,
    FileTree,
};
use std::env;
use std::path;

fn main() {
    let dir = env::args().nth(1).unwrap();
    let root = path::PathBuf::from(&dir);
    let lib_path = path::PathBuf::from(dir.clone() + "/lib.rs");

    print!("Rendering `{}/lib.rs` ... ", &dir);

    // `lib.rs`のベースとなるリストを取得
    let mut lib: FileTree = vec![];
    make_tree(&root, &mut lib, 0);

    // tree形式で表示を行う
    // display_tree(&lib);

    // ファイルに書き出し
    write_lib(&lib, &lib_path).ok();
    println!("{}", "Complete!".green().bold());
}
