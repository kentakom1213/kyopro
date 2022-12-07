
/// (深さ, パス) のベクタ
pub type FileTree = Vec<(usize, String)>;

mod make_tree;
pub use make_tree::make_tree;

mod display_tree;
pub use display_tree::display_tree;

mod write_lib;
pub use write_lib::write_lib;
