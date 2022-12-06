// # 参考
// - https://makandat.wordpress.com/2022/02/08/rust-%E3%81%AE%E5%8B%89%E5%BC%B7-%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E4%B8%80%E8%A6%A7/
// - https://qiita.com/benki/items/70ad2ee44cff9efde778

use std::path;
use colored::*;

const ROOT_DIR: &str = "..";

/// # traverse_dir
/// 
/// ## Args
/// - `path` : ルートとなるディレクトリ
/// 
/// ## Return
/// - そのディレクトリ以下にRustファイルが含まれれば`true`、含まれなければ`false`
fn traverse_dir(path: &path::PathBuf) -> bool {
    let mut is_contains_rs = false;

    for entry in path.read_dir().unwrap() {
        let next_path = entry.unwrap().path();
        let obj_name = next_path.to_str().unwrap();
        if next_path.is_dir() {
            let tmp_contains = traverse_dir(&next_path);
            if tmp_contains {
                println!("{}", obj_name.green());
            }
            is_contains_rs |= tmp_contains;
        } else {
            // Rustファイルだけを抽出
            if obj_name.ends_with(".rs") {
                println!("{}", obj_name.blue());
                is_contains_rs = true;
            }
        }
    }
    is_contains_rs
}
 
fn main() {
    let root = path::PathBuf::from(ROOT_DIR);
    traverse_dir(&root);
}
