use colored::*;

/// # display_tree
/// ファイルに書き出す
/// 
/// ## Args
pub fn display_tree(lib: &Vec<(usize, String)>) {
    println!("{}", "kyopuro".green());
    for (d, s) in lib {
        if s.ends_with(".rs") {
            println!("{} ├─ {}", " │ ".repeat(*d), s.blue());
        } else {
            println!("{} ├─ {}", " │ ".repeat(*d), s.green());
        }
    }
}
