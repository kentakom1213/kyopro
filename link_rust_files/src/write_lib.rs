use std::fs;
use std::path;
use std::io::Write;

/// # write_lib
/// libファイルを書き出す
/// 
pub fn write_lib(lib: &Vec<(usize, String)>, path: &path::PathBuf) -> bool {
    let mut libfile = fs::File::create(path).unwrap();
    
    let lib_buf = "// テスト\n\nmod test;\n".as_bytes();

    match libfile.write_all(lib_buf) {
        Ok(_) => {
            true
        },
        Err(_) => {
            false
        },
    }
}
