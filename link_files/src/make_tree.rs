use std::path;

use crate::FileTree;

/// # make_tree
/// ファイルを再帰的に探索する
/// 
/// ## Args
/// - `path` : ルートとなるディレクトリ
/// 
/// ## Return
/// - そのディレクトリ以下にRustファイルが含まれれば`true`、含まれなければ`false`
pub fn make_tree(path: &path::PathBuf, lib: &mut FileTree, depth: usize) -> bool {
    let mut is_contain = false;

    // `Cargo.toml`が含まれる場合は処理しない
    for entry in path.read_dir().unwrap() {
        let next_path = entry.unwrap().path();
        let obj_name = next_path.file_name().unwrap().to_string_lossy().to_owned().to_string();
        
        if depth >= 1 && obj_name == "Cargo.toml" {
            return false;
        }
    }

    let mut paths: Vec<_> = path.read_dir()
        .unwrap()
        .map(|r| r.unwrap().path())
        .collect();
    paths.sort();
    
    for next_path in paths {
        let obj_name = next_path.file_name().unwrap().to_string_lossy().to_owned().to_string();

        if next_path.is_dir() {
            lib.push((depth, obj_name));
            let tmp_contains = make_tree(&next_path, lib, depth+1);
            if !tmp_contains {
                lib.pop();
            }
            is_contain |= tmp_contains;
        } else {
            // Rustファイルだけを抽出
            if obj_name.ends_with(".rs") {
                lib.push((depth, obj_name));
                is_contain = true;
            }
        }
    }
    is_contain
}
