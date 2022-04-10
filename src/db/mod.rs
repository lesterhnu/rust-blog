//! db模块 数据库链接
use sea_orm::{Database,DatabaseConnection};
#[allow(dead_code)]
pub async fn get_db()->DatabaseConnection{
    Database::connect("mysql://root:123456@localhost:3306/blog").await.unwrap()
}