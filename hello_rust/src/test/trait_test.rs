use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
// #[derive(Clone, Copy, Debug、Eq, PartialEq)] // Eqを加えると、==のときの処理を実装してくれる。
struct Complex<T> {
    re: T,
    im: T,
}

// // 単一型のみサポート
// impl Add for Complex<i32> {
//     type Output = Complex<i32>;
//
//     // 戻り値は、-> Selfでもいい
//     fn add(self, rhs: Self) -> Self::Output {
//         Complex { re: self.re + rhs.re, im: self.im + rhs.im }
//     }
// }

// ジェネリック
impl<T> Add for Complex<T>
    where T: Add<Output=T>
{
    // 自分の型を返す。
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}


// // 制約を緩めたバージョン
// impl<L, R, O> Add<Complex<R>> for Complex<L>
//     where L: Add<R, Output=O>
// {
//     type Output = Complex<O>;
//
//     fn add(self, rhs: Complex<R>) -> Self {
//         Complex { re: self.re + rhs.re, im: self.im + rhs.im }
//     }
// }

// 複合代入演算子
// Addを実装しても、+=が使えない。-> AddAssignを独自に実装する必要がある。
impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    // 戻り () になる。
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}


// 等価判定
// assert_eq!(x == y, x.eq(&y()));
// assert_eq!(x != y, x.ne(&y()));

// ここに型制約を含めるか、whereを書くか。
impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

#[test]
fn test_() {
    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 5, im: 2 };
    assert_eq!(x, y);
}

#[test]
fn test_equals() {
    let s = "asadgasdf".to_string();
    let t = "gdsagdsa".to_string();
    assert_ne!(s, t); // 参照で受け取るので、下記で標準出力される。これ大事！
    println!("s: {} t: {}", s, t);
}