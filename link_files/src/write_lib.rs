use std::fs;
use std::io::BufWriter;
use std::path;
use std::io::Write;

use crate::FileTree;

const TAB: &str = "    ";

/// # write_lib
/// libファイルを書き出す
pub fn write_lib(lib: &FileTree, path: &path::PathBuf) {
    let libfile = fs::File::create(path).unwrap();
    let mut f = BufWriter::new(libfile);
    
    let mut prev_depth = 0;
    for (d, name) in lib {
        // かっこを閉じる
        for i in (*d..prev_depth).rev() {
            let line = TAB.to_string().repeat(i) + "}\n";
            f.write(&line.as_bytes()).unwrap();
        }

        let mut line = TAB.to_string().repeat(*d);

        line += &{
            if name.ends_with(".rs") {
                format!("mod {};",
                    &name[..name.len() - 3]
                )
            } else {
                format!("mod {} {{",
                    &name
                )
            }
        };

        line += "\n";

        f.write(&line.into_bytes()).unwrap();
        prev_depth = *d;
    }

    // かっこを閉じる
    for d in (0..prev_depth).rev() {
        let line = TAB.to_string().repeat(d) + "}\n";
        f.write(&line.as_bytes()).unwrap();
    }
}
