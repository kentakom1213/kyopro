// 参考：https://makandat.wordpress.com/2022/02/08/rust-%E3%81%AE%E5%8B%89%E5%BC%B7-%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E4%B8%80%E8%A6%A7/
use std::env;
use std::io;
use std::fs;
use std::vec;
use std::path;
 
/* ファイル一覧を取得する。 (ディレクトリを除く) */
fn get_files(target_dir: &str) -> io::Result<vec::Vec<path::PathBuf>> {
    // 対象のファイル一覧(io::Result<ReadDir>)を取得する。ReadDir は DirEntry を返す反復子。
    let mut entries = fs::read_dir(target_dir)?
       .map(|res| res.map(|e| e.path()))
       .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    return Ok(entries);
}

fn crawl_dir(root_dir: &str) {
    // ファイル一覧を取得して表示する。
    match get_files(root_dir) {
        Ok(v) => {
            for x in v {
                let s = x.as_path().to_str().unwrap();
                if path::Path::new(s).is_file() {
                    if s.ends_with(".rs") {
                        println!("{}", s);
                    }
                } else {
                    crawl_dir(s);  // 再帰的に検索
                }
            }
        },
        Err(s) => println!("{}", s),
    }
}
 
/* メインプログラム */
fn main() {
    println!("** ファイル一覧 (ディレクトリを除く) **");
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
 
    crawl_dir(target_dir);
}
