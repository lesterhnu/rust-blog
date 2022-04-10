use rocket::{post};
use rocket::serde::json::{Value,Json};
use rocket::serde::{Serialize,Deserialize};
use crate::response;
#[derive(Debug,Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Msg {
    pub id : i32,
    pub msg : String,
}

#[post("/add_post",data = "<msg_data>",format="json")]
pub async fn add_post(msg_data: Json<Msg>)->Value{
    let msg = msg_data.into_inner();
    println!("{:?}",msg);
    response::response_with_ok(200,"ok")
}