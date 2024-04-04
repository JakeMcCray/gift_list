/*****************************************************
 * **************************************************/

pub mod user;

#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::serde::json::Json;

use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("gift_list")]
pub struct Db(sqlx::SqlitePool);

#[get("/login")]
async fn login() -> Option<NamedFile> {
    todo!();
}

#[post("/register", format = "json", data = "<user>")]
async fn register(db: Connection<Db>, mut user: Json<user::User>) -> Result<(), ()> {
    user.register_user(db).await.map_err(|_| ())
}

#[get("/<file>")]
async fn file(file: &str) -> Option<NamedFile> {
    NamedFile::open(format!("dist/{}", file)).await.ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("dist/index.html").await.ok()
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .attach(Db::init())
        .mount("/", routes![index, login, register, file]);

    Ok(rocket.into())
}
