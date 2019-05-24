extern crate actix_web;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use actix_web::{http, server, App, Path, Responder};
use rand::Rng;
use rand::prelude::thread_rng;
use serde::Deserialize;
use std::collections::HashSet;
use std::error::Error;
use std::sync::Mutex;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct PageInfoType {
    #[allow(dead_code)]
    totalResults: u8,

    #[allow(dead_code)]
    resultsPerPage: u8
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct  ItemType {
    #[allow(dead_code)]
    kind: String,

    #[allow(dead_code)]
    etag: String,

    #[allow(dead_code)]
    id: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct YoutubeResponseType {
    #[allow(dead_code)]
    kind: String,

    #[allow(dead_code)]
    etag: String,

    #[allow(dead_code)]
    pageInfo: PageInfoType,

    #[allow(dead_code)]
    items: Vec<ItemType>
}

lazy_static! {
    static ref KEYS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref HASH_KEYS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn get_key() -> String {
    let keys = KEYS.lock().unwrap();
    keys[thread_rng().gen_range(0, keys.len())].clone()
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

    println!("DEL endpoint - deleting {} - new size {}", value, len_key());
    format!("DEL {}\n", value)
}

fn is_key_good(key: String) -> bool {
    let url: String =
        format!("https://www.googleapis.com/youtube/v3/channels?part=id&id=UC-lHJZR3Gqxm24_Vd_AJ5Yw&key={}", key);

    let resp: Result<reqwest::Response, reqwest::Error> = reqwest::get(url.as_str());
    if resp.is_err() {
        eprintln!("{} - {}", key, resp.err().unwrap().description());
        return false;
    }

    let mut resp = resp.unwrap();
    if resp.status() != 200 {
        eprintln!("{} - Received status code {}", key, resp.status());
        return false;
    }

    let result: Result<String, reqwest::Error> = resp.text();
    if result.is_err() {
        eprintln!("{} - {}", key, result.err().unwrap().description());
        return false;
    }

    let s: String = result.unwrap().clone();
    let json_obj: Result<YoutubeResponseType, serde_json::Error> = serde_json::from_str(s.as_str());

    if json_obj.is_err() {
        eprintln!("{} - {}", key, json_obj.err().unwrap().description());
        return false;
    }

    true
}

fn main() {
    println!("start");
    {
        let args: Vec<String> = std::env::args().skip(1).collect();
        println!(" Inserting {} keys", args.len());
        let mut keys = KEYS.lock().unwrap();
        for value in args {
            println!("Adding key {}", value);
            keys.push(value);
        }
    }

    std::thread::spawn(|| {
        loop {
            let keys_result = KEYS.lock().unwrap().clone();

            println!("Checking keys...");
            for i in keys_result {
                let key: String = i.clone();
                if is_key_good(key.clone()) {
                    println!("Key {} is good - keeping it", i);
                } else {
                    del_key(key);
                }

                let dur: std::time::Duration = std::time::Duration::from_secs(10);
                std::thread::sleep(dur);
            }
        }
    });

    server::new(
        || App::new()
            .route("/get", http::Method::GET, index_get)
            .route("/len", http::Method::GET, index_len)
            .route("/add/{key}", http::Method::GET, index_add)
            .route("/del/{key}", http::Method::GET, index_del))
        .bind("127.0.0.1:8080")
        .expect("Can not bind to port 8080")
        .run();
}
