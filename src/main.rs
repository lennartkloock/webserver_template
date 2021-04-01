#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use rocket::tokio;


#[tokio::main]
async fn main() -> Result<(), rocket::error::Error> {
    rocket::ignite().mount("/", routes![index]).launch().await
}

#[get("/")]
async fn index() -> String {
    String::from("Hello, world!")
}
