// Index IndexMut
use ::std::collections::HashMap;
use ::std::ops::Index;
use std::cmp::Ordering;
use std::collections::HashSet;
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

// PartialEq の意味は、浮動小数点の 0.0/0.0の結果による。
// ちなみに、x == x でtrueを返さない可能性があるものは、
// f32, f64だけ。
#[test]
fn test_partial_eq() {
    assert!(f64::is_nan(0.0 / 0.0));
    assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false); // これ、falseを返すので、partialEqと呼ばれる。
    assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);
}

// 明示的に、Eqの実装をする旨を加える。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Complex2<T> {
    re: T,
    im: T,
}


// 順序比較
#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T,
    // 含まれる
    upper: T,  // 含まれない
}

// ややこしいw
impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[test]
fn test_index() {
    let mut m = HashMap::new();
    m.insert("十", 10);
    m.insert("百", 100);
    m.insert("千", 1000);
    m.insert("万", 1_0000);
    m.insert("億", 1_000_000);

    assert_eq!(m["十"], 10);
    assert_eq!(m["千"], 1000);

    assert_eq!(*m.index("十"), 10);
    assert_eq!(*m.index("千"), 1000);
}

// IndexMut
#[test]
fn test_index_mut() {
    let mut desserts = vec!["Howalon".to_string(), "Soan papdi".to_string()];
    desserts[0].push_str("finctional");
    desserts[1].push_str("real");

    use ::std::ops::IndexMut;
    (*desserts.index_mut(0)).push_str("finctional");
    (*desserts.index_mut(1)).push_str("real");
}


struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

#[test]
fn test_my_struct_image() {
    let image = Image::<u32>::new(100, 50);
    println!("{:?}", image.pixels);

    println!("{:?}", image[3][5]); // Image[x]は、[u32]を返すので、さらに、[u32]が中身を返す。。
    println!("{:?}", image[1][1]);
}


#[test]
fn test_default() {
    let i: i8 = Default::default();
    assert_eq!(i, 0);
}


#[test]
fn test_default_trait() {
    let squeares = [4, 9, 16, 25, 36, 49, 64];
    let (power_of_two, impure): (HashSet<i32>, HashSet<i32>) = squeares
        .iter()
        .partition(|&n| n & (n - 1) == 0);

    // 2の冪乗であるかの判定。へーこれでできるんだw
    println!("{}", 9 & (9 - 1));
    println!("{}", 4 & (4 - 1));


    // assert_eq!(&power_of_two.len(), 3);
    // assert_eq!(&impure.len(), 4);

    println!("{:?}", &power_of_two);
    println!("{:?}", &impure);
}

// Cow
// 静的に確保された文字列定数 or 計算した文字列のどちらかを返す場合


struct Person {
    name: String
}

impl Person {
    pub fn new(name: impl Into<String>) -> Self {
        Person { name: name.into() }
    }
}

#[test]
fn test_into() {
    // into型にすれば、どっちでも入るようになる。
    let name: &str = "sato";
    let person = Person::new(name);

    let name: String = "sato".to_string();
    let person = Person::new(name);
}
