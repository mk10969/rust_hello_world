extern crate tokio;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

use chrono::{DateTime, Local, NaiveDateTime};
use influx_db_client::{
    Client, Point, point, Points, points, Precision, Value,
};

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


fn mock_vmstat() -> Vec<String> {
    let v = vec!["a b c g", "a b c g", "a b c g", "a b c g", "a b c g", "a b c g", "a b c g", "a b c g"];
    v.iter().map(|n| n.to_string()).collect()
}


fn read_file2(filename: String) -> Vec<String> {
    let mut file = File::open(filename).expect("not file");
    let reader = BufReader::new(file);
    reader.lines().into_iter()
        .map(|f| f.expect("not read"))
        .collect()
}


#[test]
fn test_filefile() {
    let v = read_file2("./data/test.txt".to_string());

    let client = Client::default();

    let mut c = 0;
    let mut aaa: Vec<Point> = Vec::new();

    for x in v {
        c += 1;
        let tmp = x.split_whitespace().collect::<Vec<&str>>();
        let point = Point::new("test10")
            .add_timestamp(1593469723000000000 + c * 1000000000)
            .add_tag("tags", Value::String(String::from(tmp[0])))
            .add_field("a", Value::Integer(1))
            .add_field("b", Value::Integer(1))
            .add_field("c", Value::Integer(2))
            .add_field("d", Value::Integer(3))
            .add_field("e", Value::Integer(4))
            .add_field("f", Value::Integer(5))
            .add_field("g", Value::Integer(6))
            .add_field("h", Value::Integer(7))
            .add_field("i", Value::Integer(8))
            .add_field("j", Value::Integer(9))
            .add_field("k", Value::Integer(10))
            .add_field("l", Value::Integer(11))
            .add_field("m", Value::Integer(12));
        aaa.push(point);
    }
    println!("count: {}", c);

    let points = Points::from_iter(aaa);

    tokio::runtime::Runtime::new().unwrap().block_on(async move {
        client.write_points(points, Some(Precision::Nanoseconds), None).await.unwrap();
        println!("write OK!");
    });
}

#[test]
fn test_test() {
    let client = Client::default();
    // let once_client = Client::new(Url::parse("https://example.net/a/b.html")?, "test");

    let v = read_file2("./data/test.txt".to_string());
    let aaa: Vec<Point> = v.iter()
        .map(|f| f
            .split_whitespace()
            .collect::<Vec<&str>>())
        .map(|f| Point::new("test_data")
            .add_timestamp(Local::now().timestamp())
            .add_tag("once", Value::String(f[0].to_string()))
            .add_field("number", Value::Integer(12))
            .add_field("field", Value::String(f[3].to_string())))
        .collect();

    let points = Points::from_iter(aaa);

    tokio::runtime::Runtime::new().unwrap().block_on(async move {
        client.write_points(points, Some(Precision::Seconds), None).await.unwrap();

        println!("write OK!")
    });
}


#[test]
fn influxdb_test() {
    // default with "http://127.0.0.1:8086", db with "test"
    let client = Client::default();

    let point = point!("test1")
        .add_field("foo", Value::String("bar".to_string()))
        .add_field("integer", Value::Integer(11))
        .add_field("float", Value::Float(22.3))
        .add_field("'boolean'", Value::Boolean(false));

    let point1 = Point::new("test1")
        .add_timestamp(1592469723000000000)
        .add_tag("tags", Value::String(String::from("\\\"fda")))
        .add_tag("number", Value::Integer(12))
        .add_tag("float", Value::Float(12.6))
        .add_field("fd", Value::String("'3'".to_string()))
        .add_field("quto", Value::String("\\\"fda".to_string()))
        .add_field("quto1", Value::String("\"fda".to_string()));

    let points = points!(point1, point);

    tokio::runtime::Runtime::new().unwrap().block_on(async move {
        // if Precision is None, the default is second
        // Multiple write
        client.write_points(points, Some(Precision::Seconds), None).await.unwrap();

        // query, it's type is Option<Vec<Node>>
        let res = client.query("select * from test1", None).await.unwrap();
        println!("{:?}", res.unwrap()[0].series)
    });
}

#[test]
fn test_time() {
    let no_timezone = NaiveDateTime::parse_from_str("2018-12-07 19:31:28", "%Y-%m-%d %H:%M:%S")
        .expect("parse error");
    println!("{}", &no_timezone);
    println!("{}", &no_timezone.timestamp());

    let dt = DateTime::parse_from_rfc3339("2018-12-07T19:31:28+09:00");
    println!("DateTime::parse_from_str: {:?}", &dt.unwrap());
    println!("DateTime::parse_from_str: {:?}", &dt.unwrap().timestamp());
}