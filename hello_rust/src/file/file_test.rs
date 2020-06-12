use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

#[test]
fn read_file() {
    let path = env::current_dir().unwrap();
    println!("starting dir: {}", path.display());

    let file = File::open("./data/test.txt")
        .expect("file not found!");

    BufReader::new(file).lines()
        .map(|result| result.unwrap())
        .for_each(|f| println!("{}", f));
}


#[test]
fn test_for_each() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    v.into_iter()
        .map(|i| i + 4)
        .for_each(|i| println!("{}", i));
}