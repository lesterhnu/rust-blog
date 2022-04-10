use rocket::{post};
use sea_orm::entity::prelude::*;
use rocket::serde::json::{Value,Json,serde_json::json};
use rocket::serde::{Serialize,Deserialize};
use crate::{ models, db};
use sea_orm::ActiveValue::{Set};
use time;
#[derive(Debug,Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Msg {
    pub id : i32,
    pub msg : String,
}

#[post("/add_post",data = "<msg_data>",format="json")]
pub async fn add_post(msg_data: Json<models::post::Model>)->Value{
    let msg = msg_data.into_inner();
    println!("{:?}",msg);
    let db = db::get_db().await;
    let p = models::post::ActiveModel{
        title: Set(msg.title),
        author: Set(msg.author),
        content: Set(msg.content),
        created_at: Set(time::OffsetDateTime::now_utc().unix_timestamp()),
        updated_at: Set(time::OffsetDateTime::now_utc().unix_timestamp()),
        ..Default::default()
    };
    let res  = p.insert(&db).await.unwrap();
    json!({
        "id":res.id,
    })
}