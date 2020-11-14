extern crate rayon;
use rayon::prelude::*;

use std::{
    io,
    sync::mpsc,
    thread::{self, spawn},
};

#[test]
fn test_multi_thread() {
    println!("{:?}", "multi_thread!!!");
    let handle = spawn(|| {
        println!("hello form child thread!");
    });

    handle.join().unwrap();
}

#[test]
fn test_rayon() -> io::Result<()> {
    let (v1, v2) = rayon::join(|| println!("OK"), || println!("NG"));

    (0..1000).collect::<Vec<i32>>().par_iter().for_each(|num| {
        show_number(num);
    });

    Ok(())
}

// 変数名に＆ではなく、型に＆これ！！！
fn show_number(num: &i32) {
    println!("数字: {}", num);
}

#[test]
fn test_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    // 値は{}です
    println!("Got: {}", received);
}
