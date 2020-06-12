use std::env;


#[test]
fn test_current_dir() {
    // 実行時のカレントになるんだね。
    let path = env::current_dir().unwrap();
    println!("starting dir: {}", path.display());
}