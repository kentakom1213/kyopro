use std::path;

/// # make_tree
/// 
/// ## Args
/// - `path` : ルートとなるディレクトリ
/// 
/// ## Return
/// - そのディレクトリ以下にRustファイルが含まれれば`true`、含まれなければ`false`
pub fn make_tree(path: &path::PathBuf, lib: &mut Vec<(usize, String)>, depth: usize) -> bool {
    let mut is_contain = false;
    
    for entry in path.read_dir().unwrap() {
        let next_path = entry.unwrap().path();
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
