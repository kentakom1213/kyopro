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
    let mut rust_files: Vec<String> = vec![];
    let mut rust_dirs: Vec<String> = vec![];

    for entry in path.read_dir().unwrap() {
        let next_path = entry.unwrap().path();
        let obj_name = next_path.file_name().unwrap().to_string_lossy().to_owned().to_string();
        if next_path.is_dir() {
            let tmp_contains = traverse_dir(&next_path);
            if tmp_contains {
                rust_dirs.push(obj_name);
            }
        } else {
            // Rustファイルだけを抽出
            if obj_name.ends_with(".rs") {
                rust_files.push(obj_name);
            }
        }
    }

    // 表示
    if !rust_files.is_empty() {
        println!("{}", path.to_str().unwrap().green());
    }
    for d in &rust_dirs {
        println!("{}", d.green());
    }
    for f in &rust_files {
        println!("{}", f.blue());
    }

    !rust_files.is_empty() || !rust_dirs.is_empty()
}
 
fn main() {
    let root = path::PathBuf::from(ROOT_DIR);
    traverse_dir(&root);
}
