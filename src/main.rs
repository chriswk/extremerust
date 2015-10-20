extern crate iron;
extern crate bodyparser;
extern crate regex;
extern crate persistent;
extern crate urlencoded;

use regex::Regex;
use persistent::Read;
use iron::status;
use iron::prelude::*;
use urlencoded::UrlEncodedQuery;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;
fn extremequestion(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => println!("Read body:\n{}", body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    };
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => println!("Parsed GET Request query string:\n {:?}", hashmap),
        Err(ref e) => println!("{:?}", e)
    };
    Ok(Response::with(status::Ok))
}

fn main() {
    let mut chain = Chain::new(extremequestion);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:4999").unwrap();
    println!("Running at port 4999");
}
