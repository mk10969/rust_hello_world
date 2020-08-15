use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io;
use std::iter::{once, repeat};
use std::ops::Range;
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

    people1.iter()
        .filter(|p| p.age < 20)
        .map(|p| &p.name)
        .for_each(|p| println!("{:?}", p));


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
        .inspect(|r| println!("doOnNext: {:?}", r)) // doOnNext()
        .flat_map(|r| r.1)
        .for_each(|i| println!("{:?}", i));
}


#[test]
fn test_iterator_scan() {
    (0..=20)
        .filter_map(|n|
            if n % 2 == 1 {
                Some(n * n)
            } else {
                None
            })
        .for_each(|n| println!("{}, ", n));

    // 0 ~ 10 まで実行しない。途中で打ち切ることができる。
    let iter = (0..=10)
        .scan(0, |sum, item| {
            *sum += item;
            if *sum > 10 {
                None
            } else {
                Some(item * item)
            }
        }).collect::<Vec<i32>>();

    println!("{:?}", iter);
    // assert_eq!(iter.collect::Vec<>(), vec![]);
}


#[test]
fn test_iterator_chain() {
    let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect();
    assert_eq!(v, vec![1, 2, 3, 20, 30, 40]);
}


#[test]
fn test_iterator_zip() {
    let endings = vec!["once", "twice", "chicken soup with rice"];
    let rhyme = repeat("going")
        .zip(endings).collect::<Vec<_>>();
    // println!("{:?}", rhyme);
    assert_eq!(rhyme, vec![("going", "once"),
                           ("going", "twice"),
                           ("going", "chicken soup with rice")]);

    // cloned
    let a = ['a', 'b', 'c', 'd'];
    assert_eq!(a.iter().inspect(|i| println!("{:?}", i)).next(), Some(&'a'));
    assert_eq!(a.iter().cloned().inspect(|i| println!("{:?}", i)).next(), Some('a'));
}

#[test]
fn test_iterator_cycle() {
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let v = fizzes.zip(buzzes);

    (1..=100).zip(v)
        // .inspect(|i| println!("{:?}", i))
        .map(|tuple| match tuple {
            (i, ("", "")) => i.to_string(),
            (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
        })
        .for_each(|line| print!("{}, ", line));

    fn fizzbuzz(n: u32) -> String {
        match n {
            n if n % 15 == 0 => "fizzbuzz".to_string(),
            n if n % 3 == 0 => "fizz".to_string(),
            n if n % 5 == 0 => "buzz".to_string(),
            n => n.to_string()
        }
    }
    (1..=100).map(fizzbuzz).for_each(|n| print!("{}, ", n));
}


#[test]
fn test_iter_max_min() {
    // 2つを比較する。Nanなら、panic!
    // アイテムから取り出すとき参照で、クロージャにはさらにその参照を渡すから、＆＆にする
    fn cmp(lhs: &&f64, rhs: &&f64) -> Ordering {
        lhs.partial_cmp(rhs).unwrap()
    }

    let number = [1.0, 4.0, 3.0];

    assert_eq!(number.iter().max_by(cmp), Some(&4.0));
    assert_eq!(number.iter().min_by(cmp), Some(&1.0));

    let number2 = [1.0, 4.0, std::f64::NAN, 3.0];
    // assert_eq!(number2.iter().min_by(cmp), ); //panic
}

#[test]
fn test_iter_max_min_key() {
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);

    let num = populations.iter().max_by_key(|&(key, value)| value);
    assert_eq!(num, Some((&"Portland", &583_776)));
    let num2 = populations.iter().min_by_key(|&(key, value)| value);
    assert_eq!(num2, Some((&"Greenhorn", &2)));
}

#[test]
fn test_iter_any_all() {
    let id = "Iterator";
    assert!(id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));
}

#[test]
fn test_position_rposition_exactSizeIterator() {
    let text = "Xerxes";
    // 左から検索して、ヒットしたポジション（index）を返却する    indexは 0から始まる
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'a'), None);

    // 右から検索する。
    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|c| c == &b'e'), Some(4));

    // &strは、可変長エンコーディングなため、rpositionが使えない（ExactSizeIteratorを実装していない）
    // しかし、バイト配列に対するイテレータは、配列長がわかっているので、ExactSizeIteratorを実装している
}

#[test]
fn test_iter_fold() {
    let a = [5, 6, 7, 8, 9, 10];
    assert_eq!(a.iter().fold(0, |a, _| a + 1), 6); // count
    assert_eq!(a.iter().fold(0, |a, &b| a + b), 45); //sum
    assert_eq!(a.iter().fold(1, |a, b| a * b), 151200); //product

    // max
    assert_eq!(a.iter().fold(i32::min_value(), |m, &n| std::cmp::max(m, n)), 10);

    let b = ["Pack ", "my ", "box ", "with ", "five ", "dozen."];
    let pangram = b.iter().fold(String::new(), |mut s, &i| {
        s.push_str(i); //&strだとpushできない
        s
    });
    println!("{}", pangram);
}

#[test]
fn test_iter_nth_last() {
    let mut spuares = (0..10).map(|i| i * i);
    assert_eq!(spuares.nth(4), Some(16));
    assert_eq!(spuares.nth(0), Some(25)); // 0は、next()と等価
    assert_eq!(spuares.nth(6), None); //なくなると、None

    let once = (0..10).map(|i| i * i);
    assert_eq!(once.last(), Some(81));
    // ↑これは、全部スキャンするので、iter().rev().next()がよい。
}

#[test]
fn test_iter_find() {
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);

    // 最初に見つかったもの、1つだけ返す。いまいち・・・
    println!("{:?}", populations.iter().find(|&(name, &pop)| pop < 1000));
}

#[test]
fn test_iter_partition(){
    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];
    // iterを返すのではなく、collectを返す。さらに、型指定する必要がある。上同じ型でなければならない。
    let (living, nonliving): (Vec<&str>, Vec<&str>) = things
        .iter()
        .partition(|name| name.as_bytes()[0] & 1 != 0);

    println!("{:?}", living);
    println!("{:?}", nonliving);
}

