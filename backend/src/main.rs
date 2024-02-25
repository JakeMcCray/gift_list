/*****************************************************
 *  Status:
 *
 * Was able to set up the database. Now I need code to
 * create accounts (don't forget the index function
 * contains mainly test code that should be deleted)
 * **************************************************/

mod user;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("gift_list")]
struct Db(sqlx::SqlitePool);

#[get("/")]
async fn index(mut db: Connection<Db>) -> &'static str {
    match {
        sqlx::query("SELECT name FROM users")
            .fetch_one(&mut **db)
            .await
            .unwrap()
            .is_empty()
    } {
        true => "Hello World",
        false => "cannot find anything in db",
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index])
}
