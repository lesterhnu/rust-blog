#[macro_use] extern crate rocket;
use rocket::{Request,catch};
use rocket::serde::json::{Value};
mod utils;
mod response;
mod db;
mod api;
mod models;
// mod service;
#[launch]
fn rocket()-> _{
    rocket::build()
    .register("/",catchers![not_found])
    .mount("/", routes![ping])
    .mount("/", routes![api::post::add_post])
}



#[get("/ping")]
pub fn ping() -> &'static str {
    // println!("{}",time::);
    println!("{}",utils::ConstValues::ACCESS_KEY);
    "pong"
}
#[catch(404)]
fn not_found(_req:&Request) -> Value{
    response::response_with_ok(404,"not_found")
}
