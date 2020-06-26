use std::collections::HashMap;
use std::fs::File;
use std::io::{Result, Write};

use serde::Serialize;
use serde_json;

// ジェネリックが利用できる
struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}


impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

#[test]
fn test_queue() {
    let mut q = Queue::<char>::new();
    let mut a = Queue::new(); // 型推論できる
    let mut b = Queue::new(); // 型推論できる

    q.push('A');
    a.push("asfsa");
    b.push(1);
}


// 生存期間パラメータを持てる
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}


fn find_extrema(slice: &[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least { least = &slice[i] }
        if slice[i] > *greatest { greatest = &slice[i] }
    }

    Extrema { greatest, least }
}

#[test]
fn test_find_extrema() {
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}


// トレイトを機械的に実装可能にするアノテーション
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}


// 列挙型
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty =>
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })),
            BinaryTree::NonEmpty(ref mut node) =>
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
        }
    }
}

#[test]
fn test_binary_tree() {
    let mut tree = BinaryTree::Empty;
    tree.add("AAAAA");
    tree.add("DDDDD");
    tree.add("CCCCC");
    tree.add("BBBBB");
    println!("{:?}", tree);
}

struct Sink {}

// implementが必要なものだけ、実装すればよい。
impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}


// implicit パターン
trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

// 型charに対して、IsEmojiトレイトを実装する。
impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        false
    }
}

#[test]
fn test_implicit_pattern() {
    assert_eq!('&'.is_emoji(), false);
}


struct HtmlDocument {}

trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> Result<()>;
}

// Writeを実装するすべての型:W に対して、WriteHtmlの定義を行う。
impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> Result<()> {
        Ok(())
    }
}


pub fn save_configuration(config: &HashMap<&str, i32>) -> Result<()> {
    let writer = File::create("./serialize.txt")?;
    let mut serializer = serde_json::Serializer::new(writer);
    config.serialize(&mut serializer)?;
    Ok(())
}

#[test]
fn test_serialize() {
    let mut hashmap = HashMap::new();
    hashmap.insert("a", 1);
    hashmap.insert("b", 2);
    hashmap.insert("c", 3);
    hashmap.insert("d", 4);

    // すげー implici　パターン
    save_configuration(&hashmap);
    println!("write! : {:?}", hashmap)
}

trait StringSet {
    // &selfがつかなければ、static methodとなる＜当たり前かw
    fn new() -> Self;

    fn from_slice(strings: &[&str]) -> Self;

    fn contains(&self, string: &str) -> bool;

    fn add(&mut self, string: &str);
}

#[test]
fn test_fully_qualified() {

    // 下記２つは等価
    "hello".to_string();
    str::to_string("hello"); //こっちで呼んだ方がいいことが多いみたい！

    let zero = 0;
    // zero.abs(); // error method 'abs' not found.
    i64::abs(zero);
}
