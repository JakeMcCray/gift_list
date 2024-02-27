/*****************************************************
 *  Status:
 *      No bugs i am aware of, login function left unfinished
 *
 *  Todo:
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

use user::UserError;

#[macro_use]
extern crate rocket;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("gift_list")]
pub struct Db(sqlx::SqlitePool);

#[get("/")]
async fn index(mut db: Connection<Db>) -> &'static str {
    match user::User::new(
        db,
        "John Doe",
        &Some("email@gmail.com".to_string()),
        "Password123",
    )
    .await
    {
        Ok(_) => "User Succefully created",
        Err(UserError::UsernameExists) => "user with that username already exists",
        Err(UserError::EmailExists) => "user with that email already exists",
        Err(UserError::DatabaseError) => "There was a problem accesing the database",
        Err(_) => "An error occured",
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index])
}
