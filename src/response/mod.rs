use rocket::serde::json::{Value,serde_json::json};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<'a,T> {
    pub code: i32,
    pub msg: &'a str,
    pub data: T
}

pub fn response_with_ok<'a>(code:i32,msg:&'a str)->Value{
    json!(Response{ code, msg, data: "asdf" })
}