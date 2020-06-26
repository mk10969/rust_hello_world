use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;

#[test]
fn test_iter() {
    let mut l = Vec::new();
    l.push(1);
    l.push(2);
    l.push(3);
    l.push(4);

    // iter()は、所有権を奪わない。共有参照するだけ。
    l.iter().for_each(|i| println!("{}", i));

    // 共有参照で、into_iter()を呼ぶと、所有権を奪わない。
    (&l).into_iter().for_each(|i| println!("{}", i));

    // collection.into_iter()で、呼ぶと所有権を奪う。
    l.into_iter().for_each(|i| println!("{}", i));
    // 以降、lは使えない。
}


fn dump<T, U>(t: T)
    where T: IntoIterator<Item=U>,
          U: Debug
{
    for i in t {
        println!("{:?}", i)
    }
}


#[test]
fn test_iter2() {
    let l = vec![1, 2, 3, 4, 5, 6];
    dump::<Vec<i32>, i32>(l); // moveする
}


#[test]
fn test_some_iterator() {
    // Range
    let a = 1..10;

    // Option
    let b = Option::Some("aaa");
    b.iter().for_each(|i| println!("{}", i));
    println!("=================");

    // Result
    // Ok("aa").iter().for_each(|i| println!("{}", i));

    // Vec
    let c = vec![1, 2, 3, 4, 5, 6];
    c.windows(3).for_each(|i| println!("{:?}", i)); // オーバラップする
    println!("=================");
    c.chunks(4).for_each(|i| println!("{:?}", i)); // オーバラップしない
    println!("=================");
    let mut c2 = vec![1, 2, 3, 4, 5, 6];
    c2.chunks_mut(10).for_each(|i| println!("{:?}", i)); // mut
    println!("=================");

    // String &str
    let c3 = "abcddddefgh".to_string();
    c3.split(|i| i == 'd').for_each(|i| println!("{:?}", i)); // dはなくなる
    println!("=================");
    c3.rsplit(|i| i == 'd').for_each(|i| println!("{:?}", i)); // dはなくなる and reverse
    println!("=================");
    c3.splitn(3, 'd').for_each(|i| println!("{:?}", i)); // dはなくなる and 3つまで分割
    println!("=================");
    c3.matches(char::is_alphabetic).for_each(|i| println!("{:?}", i));

    // HashMap BtreeMap
    let map = [("roo", 1), ("bar", 2), ("baz", 3)]
        .iter()
        .cloned()
        .collect::<HashMap<&str, i32>>();
    println!("=================");
    map.keys().for_each(|i| println!("{:?}", i));
    println!("=================");
    map.values().for_each(|i| println!("{:?}", i));


    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Person {
        name: String,
        age: u32,
    }
    // HashSet BtreeSet
    let people1 = vec![
        Person { name: "test1".into(), age: 10 },
        Person { name: "test2".into(), age: 20 },
        Person { name: "test3".into(), age: 30 },
        Person { name: "test1".into(), age: 10 },
        Person { name: "test2".into(), age: 10 },
    ];
    let people2 = vec![
        Person { name: "test1".into(), age: 10 }
    ];

    let set1 = people1.into_iter().collect::<HashSet<Person>>();
    let set2 = people2.into_iter().collect::<HashSet<Person>>();
    println!("=================");
    // 和集合
    set1.union(&set2).into_iter().for_each(|i| println!("{:?}", i));
    println!("=================");
    // 積集合
    set1.intersection(&set2).into_iter().for_each(|i| println!("{:?}", i));
}

#[test]
fn test_iterator2() {
    // map and filter
    let text = "  string  \n  integer  \n  map  \n 123  \n exception".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "integer")
        .collect();
    println!("{:?}", v);
    println!("{}", text); // moveされない
    println!("=================");

    // filter_map and flat_map
    text.split_whitespace()
        .filter_map(|s| i32::from_str(s).ok())
        .for_each(|i| println!("{:?}", i));

    println!("=================");
    // もし、filterとmapを使う場合は下記のようになる
    // なるほど、rustの場合、filter_mapが有効になるね！
    text.split_whitespace()
        .map(|s| i32::from_str(s))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .for_each(|i| println!("{:?}", i));


    // flat_map
    let mut flat_map_test = HashMap::new();
    flat_map_test.insert("roo", vec![1, 2, 3, 4]);
    flat_map_test.insert("bar", vec![5, 7, 8, 9]);
    flat_map_test.insert("baz", vec![10, 11, 12, 13]);
    println!("=================");
    flat_map_test.iter()
        .flat_map(|r| r.1)
        .for_each(|i| println!("{:?}", i));
}


