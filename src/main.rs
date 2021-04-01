/*
 * Copyright (c) 2021, PockelHockel, All rights reserved.
 */

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

#[tokio::main]
async fn main() -> Result<(), rocket::error::Error> {
    rocket::ignite().mount("/", routes![index]).launch().await
}

#[get("/")]
async fn index() -> String {
    String::from("Hello, world!")
}
