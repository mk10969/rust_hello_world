extern crate hello_rust;
extern crate iron;
extern crate urlencoded;

use crate::iron::Plugin;
use std::str::FromStr;

use hyper::header::ContentType;
use iron::{Iron, IronResult, Request, Response, Set, status};
use router::Router;
use urlencoded::UrlEncodedBody;

fn main() {
    println!("Server on http://localhost:4000...");

    let mut router = Router::new();

    router.get("/", handler, "root");
    router.get("gcd", get_form, "get-gcd");
    router.post("/gcd", post_gcd, "post-gcd");

    Iron::new(router).http("localhost:4000").unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world!")))
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    res.set_mut(status::Ok);
    res.headers.set(ContentType::html());
    res.set_mut(r#"
           <title>GCD Calculator</title>
           <form action="/gcd" method="post">
            <input type="text" name="n">
            <input type="text" name="n">
            <button type="submit">Comute GCD</botton>
           </form>
    "#);
    Ok(res)
}

fn post_gcd(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();

    let form_data = match req.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            res.set_mut(status::BadRequest);
            res.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(res);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            res.set_mut(status::BadRequest);
            res.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(res);
        }
        Some(num) => num
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                res.set_mut(status::BadRequest);
                res.set_mut(format!("Value for 'n' parameter not a number: {:?}\n", &unparsed));
                return Ok(res);
            }
            Ok(n) => { numbers.push(n); }
        };
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = hello_rust::test::hello_world_1::gcd(d, *m);
    }

    res.set_mut(status::Ok);
    res.headers.set(ContentType::html());
    res.set_mut(
        format!("Thr greatest common divisor of the numbers {:?} is <b>{}</b>\n", numbers, d));
    Ok(res)
}
