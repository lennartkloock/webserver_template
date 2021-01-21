/*
 * Copyright (c) 2021, PockelHockel, All rights reserved.
 */

//! TODO: `rustup default nightly`

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> String {
    String::from("Hello, world!")
}
