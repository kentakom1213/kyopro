// # 参考
// - https://makandat.wordpress.com/2022/02/08/rust-%E3%81%AE%E5%8B%89%E5%BC%B7-%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E4%B8%80%E8%A6%A7/
// - https://qiita.com/benki/items/70ad2ee44cff9efde778

use std::path;
use colored::*;

const ROOT_DIR: &str = "..";

fn traverse_dir(path: &path::PathBuf) {
    for entry in path.read_dir().unwrap() {
        let next_path = entry.unwrap().path();
        let obj_name = next_path.to_str().unwrap();
        if next_path.is_file() {
            // Rustファイルだけを抽出
            if obj_name.ends_with(".rs") {
                println!("{}", obj_name.blue());
            }
        } else {
            println!("{}", obj_name.green());
            traverse_dir(&next_path);
        }
    }
}
 
fn main() {
    let root = path::PathBuf::from(ROOT_DIR);
    traverse_dir(&root);
}
