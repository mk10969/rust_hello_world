use std::collections::HashMap;
use std::thread;

struct City {
    name: String,
    population: i64,
    country: String,
}

impl City {
    fn get_statistic(&self, i: i64) -> i64 {
        i + 1
    }
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|c| c.population)
}

fn sort_cities2(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| city.get_statistic(1))
}

#[test]
fn test_thread() {
    let mut cities = vec![
        City {
            name: "aa".to_string(),
            population: 3,
            country: "jpn".to_string(),
        },
        City {
            name: "bb".to_string(),
            population: 1,
            country: "amr".to_string(),
        }
    ];
    start_sorting_thread(cities, 1);
}

fn start_sorting_thread(mut cities: Vec<City>, i: i64) -> thread::JoinHandle<Vec<City>> {
    // クロージャに借用させるのではなく、移動するよう命じる。

    let key_fn = move |city: &City| -> i64 {
        -city.get_statistic(i)
    };

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })

    // 当たり前だが、moveしたら、再度アクセスすることができなくなる。
    // しかし、i32のように移動ではなくコピー型の場合、コピーが行われるため。
    // 再度アクセス可能となる。。。＜
}


// fn(&City) -> bool  fn型       // function
// Fn(&City) -> bool  Fnトレイト  // function and closure
fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
    where F: Fn(&City) -> bool
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1
        }
    }
    count
}


// クロージャを２度呼ぶ
#[test]
fn test_2time_closure() {
    call_twice(|| print!("test !!!"));

    // ２回目は、my_strが移動しているので、利用できない。
    // let my_str = "hello".to_string();
    // let f = || drop(my_str);
    // call_twice(f);
}


fn call_twice<F>(closure: F) where F: Fn() {
    closure();
    closure();
}

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}


struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct BasicRouter<C>
    where C: Fn(&Request) -> Response
{
    routers: HashMap<String, C>
}

impl<C> BasicRouter<C>
    where C: Fn(&Request) -> Response
{
    fn new() -> Self {
        BasicRouter {
            routers: HashMap::new()
        }
    }

    fn add_route(&mut self, url: &str, call_back: C) {
        self.routers.insert(url.to_string(), call_back);
    }
}

fn get_form_response() -> Response {
    Response {
        code: 111,
        headers: HashMap::new(),
        body: Vec::new()
    }
}

// 型の方に、&をつけて、参照の借用を宣言する！
fn get_form_response2(req: &Request) -> Response {
    Response {
        code: 222,
        headers: HashMap::new(),
        body: req.body.clone()
    }
}


#[test]
fn test_my_router() {
    let mut router = BasicRouter::new();
    router.add_route("/", |_| get_form_response());
    // expected closure, found a different closure
    // さまざまな型をサポートしたいのであれば、ボックスかトレイトオブジェクトを使えば良い！
    // router.add_route("/test", |&req| get_form_response2(req));
}

type BoxedCallBack = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter2 {
    routers: HashMap<String, BoxedCallBack>
}

impl BasicRouter2 {
    fn new() -> Self {
        BasicRouter2 {
            routers: HashMap::new()
        }
    }

    fn add_route<C>(&mut self, url: &str, call_back: C)
    // 生存期間をstaticにして、スコープが離れても大丈夫にする。
        where C: Fn(&Request) -> Response + 'static
    {
        self.routers.insert(url.to_string(), Box::new(call_back));
    }
}

#[test]
fn test_my_router2() {
    let mut router = BasicRouter2::new();
    router.add_route("/", |_| get_form_response());
    // expected closure, found a different closure
    // さまざまな型をサポートしたいのであれば、ボックスかトレイトオブジェクトを使えば良い！
    router.add_route("/test", |req| get_form_response2(req));
}