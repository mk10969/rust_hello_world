use io::{BufRead, BufReader};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io,
};

#[test]
fn test_collection_entry() {
    let mut map: HashMap<String, i32> = HashMap::new();

    let ballots: Vec<&str> = vec!["a", "b", "a", "b", "a", "b", "c", "c", "a", "d", "c"];

    for name in &ballots {
        let count = map.entry(name.to_string()).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map)
    // これは便利だな。
}

#[test]
fn test_number() {
    println!("{:?}", 'F'.to_digit(10));
}

#[test]
fn test_reader() {
    // stdinは、参照を保持しなければならない。
    // io::stdin().lock()とは、書けない。
    let stdin = io::stdin();
    grep("AAA", stdin.lock()).unwrap();

    let f = File::open("/opt/aaaa.txt").unwrap();
    grep("AAA", BufReader::new(f)).unwrap();
}

#[test]
fn test_reader2_collect() {
    // これはほしいものではない。。。
    // let results: Vec<io::Result<String>> = reader.lines().collect();
    // Result型は、FromIteratorの実装を見てみると、下記のように書くことができる。素晴らしい！
    // let results = reader.lines().collect::<io::Result<Vec<String>>>()?;

    // Result型すべてに、適応できるので、無駄なコードが消えるよ。
}

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{:?}", line);
        }
    }
    Ok(())
}

// filey read and write
#[test]
fn test_file1() {
    // OpenOptionsという便利なものがあるので、それを利用してファイルをopenした方がよい。

    // ファイルが存在したら、後ろに追記する。
    let log = OpenOptions::new().append(true).open("server.log").unwrap();

    // ファイルが存在したら、失敗する。かつwrite可能
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("new_file.txt")
        .unwrap();
}
