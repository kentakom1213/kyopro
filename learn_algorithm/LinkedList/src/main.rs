/// # linked_list
/// 連結リスト作成用マクロ
macro_rules! linked_list {
    ( $val:expr, $( $vals:expr ), * $(,)* ) => {{
        LinkedList::Node {
            val: $val,
            next: Box::new( linked_list!( $( $vals, )* ) ),
        }
    }};
    () => {{
        LinkedList::Null   
    }};
}

/// # LinkedList
/// 連結リストのノード
#[derive(Debug)]
enum LinkedList<T> {
    Null,
    Node {
        val: T,
        next: Box<LinkedList<T>>,
    }   
}

impl<T> LinkedList<T>
where
    T: std::cmp::Eq
{

}


fn main() {
    let list2 = linked_list!(1, 2, 3, 4);

    println!("{:?}", list2);
}
