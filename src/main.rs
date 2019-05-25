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
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;

type Key = HashMap<String, bool>;

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
    static ref KEYS: Mutex<Key> = Mutex::new(HashMap::new());
}

fn get_key() -> String {
    let keys: Key = KEYS.lock().unwrap().clone();
    let cloned_keys: Vec<String> = {
        let mut cloned_keys: Vec<String> = Vec::new();

        for k in keys {
            if k.1 {
                cloned_keys.push(k.0.clone());
            }
        }

        cloned_keys
    };

    if cloned_keys.is_empty() {
        String::new()
    } else {
        cloned_keys[thread_rng().gen_range(0, cloned_keys.len())].clone()
    }
}

fn index_get(_info: Path<()>) -> impl Responder {
    let key: String = get_key();

    println!("GET {}", key);
    format!("{}", key)
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

    println!("Key {} is good - keeping it", key);
    true
}

fn main() {
    println!("start");
    {
        let init_keys: Vec<String> = std::env::args().skip(1).collect();
        println!(" Inserting {} keys", init_keys.len());

        for k in init_keys {
            KEYS.lock().unwrap().insert(k.clone(), is_key_good(k));
        }
    }

    std::thread::spawn(move  || {
        loop {
            let keys_result: Key = KEYS.lock().unwrap().clone();

            println!("Checking keys...");
            for i in keys_result {
                let key: String = i.0.clone();
                let value: bool = is_key_good(key.clone());
                KEYS.lock().unwrap().insert(key, value);

                let dur: std::time::Duration = std::time::Duration::from_secs(10);
                std::thread::sleep(dur);
            }
        }
    });

    server::new(
        || App::new()
            .route("/get", http::Method::GET, index_get))
        .bind("127.0.0.1:8080")
        .expect("Can not bind to port 8080")
        .run();
}
