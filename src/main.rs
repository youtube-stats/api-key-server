extern crate actix_web;
extern crate rand;

use actix_web::{http, server, App, Path, Responder};
use rand::seq::SliceRandom;
use std::cell::RefCell;

thread_local! {
    pub static KEYS: RefCell<Vec<String>> = RefCell::new(vec![]);
}

fn get_key() -> String {
    let mut rng: rand::prelude::ThreadRng = rand::prelude::thread_rng();

    KEYS.with(|keys: &RefCell<Vec<String>>| {
        keys.borrow().choose(&mut rng).unwrap().clone()
    })
}

fn len_key() -> usize {
    KEYS.with(|keys: &RefCell<Vec<String>>| {
        keys.borrow().len()
    })
}

fn add_key(value: String) {
    KEYS.with(|keys: &RefCell<Vec<String>>| {
        keys.borrow_mut().push(value)
    })
}

fn del_key(value: String) {
    KEYS.with(|keys: &RefCell<Vec<String>>| {
        let index: usize = keys.borrow().iter().position(|x| *x == value).unwrap();
        keys.borrow_mut().remove(index);
    })
}

fn index_get(_info: Path<()>) -> impl Responder {
    let key: String = get_key();

    println!("GET {}", key);
    format!("{}", key)
}

fn index_len(_info: Path<()>) -> impl Responder {
    let len: usize = len_key();

    println!("LEN {}", len);
    format!("{}", len)
}

fn index_add(info: Path<(String)>) -> impl Responder {
    let value: String = info.into_inner();
    add_key(value.clone());

    println!("ADD endpoint - adding {}", value);
    format!("ADD {}\n", value)
}

fn index_del(info: Path<(String)>) -> impl Responder {
    let value: String = info.into_inner();
    del_key(value.clone());

    println!("DEL endpoint - deleting {}", value);
    format!("DEL {}\n", value)
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