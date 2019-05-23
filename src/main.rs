extern crate actix_web;
#[macro_use]
extern crate lazy_static;
extern crate rand;

use std::collections::HashSet;
use std::sync::Mutex;
use actix_web::{http, server, App, Path, Responder};
use rand::prelude::{ThreadRng, thread_rng};
use rand::seq::SliceRandom;

lazy_static! {
    static ref KEYS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref HASH_KEYS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn get_key() -> String {
    let mut rng: ThreadRng = thread_rng();

    KEYS.lock().unwrap().choose(&mut rng).unwrap().clone()
}

fn len_key() -> usize {
    KEYS.lock().unwrap().len()
}

fn add_key(value: String) {
    if !HASH_KEYS.lock().unwrap().contains(&value) {
        KEYS.lock().unwrap().push(value.clone());
        HASH_KEYS.lock().unwrap().insert(value);
    }
}

fn del_key(value: String) {
    if HASH_KEYS.lock().unwrap().contains(&value) {
        let index: usize = KEYS.lock().unwrap().iter().position(|x| *x == value).unwrap();
        KEYS.lock().unwrap().remove(index);
    }
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

    println!("ADD endpoint - adding {} - new size {}", value, len_key());
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
            .route("/get", http::Method::GET, index_get)
            .route("/len", http::Method::GET, index_len)
            .route("/add/{key}", http::Method::GET, index_add)
            .route("/del/{key}", http::Method::GET, index_del))
        .bind("127.0.0.1:8080")
        .expect("Can not bind to port 8000")
        .run();
}