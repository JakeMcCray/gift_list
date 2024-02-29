/*****************************************************
 *  Status:
 *      Just found out about the map_err function.
 *      I should probably simplify a lot of my code
 *      by using that
 *
 *  Todo:
 *      -Finish register function
 *      -Finish login funtion
 *      -Create Mutex lock for log
 *
 *
 *  Other:
 *  -remember to change the lsp settings to map
 *      :LspDocumentDiagnostics to somtihng and
 *      to get rid of the default diagnostics
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
    //TODO: check if user is already logged in
    NamedFile::open("login.html").await.ok()
}

#[post("/login", data = "<user>")]
async fn register(db: Connection<Db>, user: Json<user::User>) -> Option<NamedFile> {
    if let Ok(_) = user::User::verify_login(db, &*user).await {
        NamedFile::open("success.html").await.ok()
    } else {
        todo!()
    }
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("homepage.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index, login])
        .mount("/login", routes![register])
}
