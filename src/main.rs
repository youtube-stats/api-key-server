extern crate actix_web;
use actix_web::{http, server, App, Path, Responder};


fn index_get(_info: Path<()>) -> impl Responder {
    let string: &str = "GET";

    println!("{}", string);
    format!("{}", string)
}

fn index_add(info: Path<(String)>) -> impl Responder {
    let string: &str = "ADD";

    println!("{} endpoint - adding {}", string, info.into_inner());
    format!("{}", string)
}

fn index_del(info: Path<(String)>) -> impl Responder {
    let string: &str = "DEL";

    println!("{} endpoint - deleting {}", string, info.into_inner());
    format!("{}", string)
}

fn main() {
    server::new(
        || App::new()
            .resource("/get", |r| r.method(http::Method::GET).with(index_get))
            .resource("/add/{key}", |r| r.method(http::Method::GET).with(index_add))
            .resource("/del/{key}", |r| r.method(http::Method::GET).with(index_del))
    ).bind("127.0.0.1:8080")
        .expect("Can not bind to port 8000")
        .run();
}