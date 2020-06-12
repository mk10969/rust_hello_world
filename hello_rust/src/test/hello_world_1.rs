use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;
use std::str::FromStr;

#[test]
fn test_hello_world() {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error persing argumanet"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER....").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("THe greatest common divisor of {:?} is {}", numbers, d)
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    println!("finish!")
}


#[test]
fn test() {
    let s = "a b c";
    for f in s.chars() {
        println!("{}", f)
    }
}

#[test]
fn test2() {
    let a: Vec<String> = vec!["0".to_string()];
    for f in a {
        println!("{}", f)
    }
}


// 配列
#[test]
fn array() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer.len(), 6);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            };
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 1, 2, 4];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

// ベクトル
#[test]
fn vector() {
    // vec マクロで生成
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

    // Vecで初期化
    let mut v2: Vec<i32> = Vec::new();
    assert_eq!(v2.len(), 0);
    // collect()メソッドで生成
    let v3: Vec<i32> = (0..5).collect();
    assert_eq!(v3, [0, 1, 2, 3, 4]);

    let mut v4 = vec!["a man", "a plan", "a canal", "panama"];
    v4.reverse();
    assert_eq!(v4, vec!["panama", "a canal", "a plan", "a man"]);

    let mut v5 = Vec::with_capacity(2);
    v5.push(1);
    v5.push(2);
    assert_eq!(v5.len(), 2);
    assert_eq!(v5.capacity(), 2);
    v5.push(3);
    v5.push(4);
    assert_eq!(v5.len(), 4);
    assert_eq!(v5.capacity(), 4);
}

// スライス
#[test]
fn slice() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print(&v);
    print(&a);

    print(&v[0..2]);
    print(&a[2..]);
    print(&sv[1..3]);
}

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
    println!("--------------")
}


// 文字列
#[test]
fn test_string() {
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}", speech);

    // 全部繋がる
    println!("aaaaaaaaaaaaaaaa,
    bbbbbbbbbbbbbbbbb");

    // バックスペースを入れると、改行コードと先頭のスペースが省略される。
    println!("aa, and \
    bbb -\
    cccc.");

    // raw string
    println!(r"C:\program Files\Gorilla");
    println!(r"\d+(\.\d+)*");

    // ダブルクォートを含めるraw string
    println!(r###"aaa " bbb " ccc"###)
}

// バイト文字列
#[test]
fn test_byte_string() {
    let method = b"GET";
    assert_eq!(method.len(), 3);
    assert_eq!(method, &[b'G', b'E', b'T']);
}

#[test]
fn test_move() {
    let mut s = "Govinda".to_string();
    s = "Siddharha".to_string(); // ↑ dropされる。
    let mut t = s;
    s = "aaaaa".to_string(); // ↑ dropされない。moveしたから。
}

#[test]
fn test_move_control() {
    let x = vec![10, 20, 30];
    if true {
        f(x);
    }
//    g(x); // moved -> compile error!
}

#[test]
fn test_move_control2() {
    let x = vec![10, 20, 30];
    // loop {
    //     f(x); // moved -> compile error!
    // }
}

#[test]
fn test_move_control3() {
    let mut x = vec![10, 20, 30];
    loop {
        f(x);
        x = vec![10, 20, 30];
        break;
    }
    g(x); // xに初期化しているから、Not compile error!
}

#[test]
fn test_move_control4() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    // [101, 102, 103, 104, 105]
    // ベクトルの中身を、一般的な言語のような表現で、移動することができない。
    // let third = v[2]; // cannot move

    // 参照をすればいい
    let third = &v[2];
    println!("{}", third);

    // ケツを取り出す
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // indexで指定した要素を取り出す。その後、最後の要素を取り除いたindexにmoveする。
    let second = v.swap_remove(0);
    println!("{}", second);
    assert_eq!(second, "101");

    println!("{:?}", v);

    // replace
    let once = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(once, "103");

    assert_eq!(v, vec!["104", "102", "substitute"]);
    println!("{:?}", v);
}

#[test]
fn test_vec() {
    // 基本ベクトルのindexを変更できないが、for文の中では変更することができる。

    let v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    for mut x in v {
        x.push('!'); // mut xにすれば、所有権を奪えるので変更可能になる。
        println!("{}", x);
    };

    // println!("{:?}", v); // after move
}

#[test]
fn test_vec_index_stuct() {
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Manako".to_string()),
        birth: 1525,
    });

    // let first_name = composers[0].name; // indexされた値の移動はできない。

    // Optionを利用して、take()して、replaceすることができる。
    let first_name = composers[0].name.take();
    println!("{:#?}", first_name);
    println!("{:#?}", composers); // #は、pprintの意味
}


#[test]
fn test_struct_default_not_copy() {
    struct Label {
        number: u32
    }

    fn print(l: Label) { println!("STAMP: {}", l.number); }

    let l = Label { number: 3 };
    print(l);
    // println!("My label number is : {}", l.number); // moveされる
}

#[test]
fn test_copy() {
    #[derive(Copy, Clone)]
    struct Label {
        number: u32
    }

    fn print(l: Label) { println!("STAMP: {}", l.number); }

    let l = Label { number: 3 };
    print(l); //ここでmoveされず、Copyされる！
    println!("My label number is : {}", l.number); // copyされるので表示できる。
}


// #[test]
// fn test_copy2(){
//     #[derive(Copy, Clone)]
//     struct Label {
//         number: String // this field does not implement `Copy`
//     }
// }


fn f(x: Vec<i32>) {
    println!("{:?}", x);
}

fn g(x: Vec<i32>) {
    println!("{:?}", x);
}


#[test]
fn test_rc_and_arc() {
    let s: Rc<String> = Rc::new("ABC".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("A"));
    assert_eq!(t.find("C"), Some(2)); // find 見つかった場所のindex
    print!("{}", u);

    // cannot borrow as mutable
    // mutをつけても、immutableとして扱われる。
    // s.push_str("GGG");
}


#[test]
fn test_reference() {
    // alias
    type Table = HashMap<String, Vec<String>>;

    fn show(table: Table) {
        for (food, values) in table {
            println!("Food: {}", food);
            for value in values {
                println!("Value: {}", value);
            };
        }
    }

    let mut table = Table::new();
    table.insert("apple".to_string(), vec!["aaa".to_string(), "bbb".to_string()]);
    table.insert("banana".to_string(), vec!["aaa".to_string(), "bbb".to_string()]);
    table.insert("candy".to_string(), vec!["aaa".to_string(), "bbb".to_string()]);

    // 所有権が奪われるので、tableは破壊される。
    show(table);
}

#[test]
fn test_shared_reference() {
    // alias
    type Table = HashMap<String, Vec<String>>;

    fn show(table: &Table) {
        for (food, values) in table {
            println!("Food: {}", food);
            for value in values {
                println!("Value: {}", value);
            };
        }
    }

    let mut table = Table::new();
    table.insert("apple".to_string(), vec!["aaa".to_string(), "bbb".to_string()]);
    table.insert("banana".to_string(), vec!["aaa".to_string(), "bbb".to_string()]);
    table.insert("candy".to_string(), vec!["aaa".to_string(), "bbb".to_string()]);

    // 借用されるだけなので、tableは破壊されない。
    // shared referenceが存在する場合、tableは所有者でさえ変更不可能になる。
    show(&table);
}


#[test]
fn test_mutable_reference() {
    // alias
    type Table = HashMap<String, Vec<String>>;

    fn sort(table: &mut Table) {
        for (_f, values) in table {
            values.sort();
        };
    }

    fn show(table: &Table) {
        for (food, values) in table {
            println!("Food: {}", food);
            for value in values {
                println!("Value: {}", value);
            };
        }
    }

    let mut table = Table::new();
    table.insert("apple".to_string(), vec!["aaa".to_string(), "bbb".to_string()]);
    table.insert("banana".to_string(), vec!["bbb".to_string(), "aaa".to_string()]);
    table.insert("candy".to_string(), vec!["bbb".to_string(), "aaa".to_string()]);

    sort(&mut table);
    show(&table);
}


#[test]
fn test_reference2() {
    let x = 10;
    let y = 20;
    let mut r = &x;

    if true {
        r = &y;
    }
    println!("r: {}", r); // *なくても暗黙で参照解決してくれる。
    println!("x: {}", x);
    println!("y: {}", y);

    assert_eq!(*r, 20); // *ないと、ダメ！.演算子使わないから。
    assert_eq!(x, 10);
    assert_eq!(y, 20);
}


#[test]
fn test_reference3() {
    struct Point { x: i32, y: i32 }

    let point = Point { x: 1000, y: 729 };
    let r = &point;
    let rr = &r;
    let rrr = &rr;

    println!("{}", rrr.y);
    assert_eq!(rrr.y, 729);
}

#[test]
fn test_reference4() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    // their reference are equal
    assert!(rx == ry);
    // but occupy different addresses
    assert!(!std::ptr::eq(rx, ry));
}