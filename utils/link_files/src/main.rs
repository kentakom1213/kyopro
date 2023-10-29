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
    let root = path::PathBuf::from(env::var("KYOPURO_ROOT_DIR").unwrap());
    let lib_path = path::PathBuf::from(env::var("KYOPURO_LIB_PATH").unwrap());

    print!("{}", "Rendering `lib.rs` ... ".blue());

    // `lib.rs`のベースとなるリストを取得
    let mut lib: FileTree = vec![];
    make_tree(&root, &mut lib, 0);

    // tree形式で表示を行う
    // display_tree(&lib);

    // ファイルに書き出し
    write_lib(&lib, &lib_path);

    println!("{}", "Complete!".green().bold());
}
