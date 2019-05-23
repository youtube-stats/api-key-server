extern crate actix_web;

#[macro_use]
extern crate lazy_static;
extern crate rand;

use actix_web::{http, server, App, Path, Responder};
use rand::seq::SliceRandom;

lazy_static! {
    static ref KEYS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(vec![]);
}

fn index_get(_info: Path<()>) -> impl Responder {
    let mut rng: rand::prelude::ThreadRng = rand::prelude::thread_rng();
    let keys: Vec<String> = KEYS.lock().unwrap().to_vec();
    let string: String = keys.choose(&mut rng).unwrap().clone();

    println!("{}", string);
    format!("{}", string)
}

fn index_len(_info: Path<()>) -> impl Responder {
    let len: usize = KEYS.lock().unwrap().to_vec().len();

    println!("LEN");
    format!("{}", len)
}

fn index_add(info: Path<(String)>) -> impl Responder {
    let value: String = info.into_inner();
    let mut keys: Vec<String> = KEYS.lock().unwrap().to_vec();
    keys.push(value);

    println!("ADD endpoint - adding {}", keys.last().unwrap());
    format!("ADD\n")
}

fn index_del(info: Path<(String)>) -> impl Responder {
    let val: String = info.into_inner();
    let mut xs: Vec<String> = KEYS.lock().unwrap().to_vec();
    let index = xs.iter().position(|x| *x == val).unwrap();
    xs.remove(index);

    println!("DEL endpoint - deleting {}", val);
    format!("DEL\n")
}

fn main() {


    server::new(
        || App::new()
            .resource("/get", |r| r.method(http::Method::GET).with(index_get))
            .resource("/len", |r| r.method(http::Method::GET).with(index_len))
            .resource("/add/{key}", |r| r.method(http::Method::GET).with(index_add))
            .resource("/del/{key}", |r| r.method(http::Method::GET).with(index_del))
    ).bind("127.0.0.1:8080")
        .expect("Can not bind to port 8000")
        .run();
}