// # 参考
// - https://makandat.wordpress.com/2022/02/08/rust-%E3%81%AE%E5%8B%89%E5%BC%B7-%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E4%B8%80%E8%A6%A7/
// - https://qiita.com/benki/items/70ad2ee44cff9efde778

#![allow(dead_code)]

use std::env;
use std::path;
use colored::*;

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
 
/* メインプログラム */
fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut target_dir: &str = ".";  // 対象のディレクトリ
    // let mut input = String::new();  // 作業用
 
    // コマンドライン引数があるか確認して、ない場合はキーボード入力を促す。
    if argv.len() < 2 {
        // println!("対象のディレクトリを入力 >");
        // match io::stdin().read_line(&mut input) {
        //     Ok(n) => {
        //         if n == 0 {  // 文字列長がゼロなら終了する。
        //             panic!();
        //         }
        //         target_dir = input.as_mut_str();
        //     },
        //     Err(error) => panic!("error: {}", error),  // エラーなら終了する。
        // }
    }
    else {
        target_dir = argv[1].as_str();
    }
 
    // crawl_dir(target_dir);
    let target_path = path::PathBuf::from(&target_dir);
    traverse_dir(&target_path);
}
