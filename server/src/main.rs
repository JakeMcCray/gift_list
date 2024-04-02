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
async fn register(db: Connection<Db>, user: Json<user::User>) -> Option<NamedFile> {
    if let Ok(_) = user::User::verify_login(db, &*user).await {
        NamedFile::open("success.html").await.ok()
    } else {
        todo!()
    }
}

#[get("/<file>")]
async fn file(file: &str) -> Option<NamedFile> {
    NamedFile::open(format!("../ui/dist/{}", file)).await.ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("../ui/dist/index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index, login, register, file])
}
