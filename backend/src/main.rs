/*****************************************************
 *  Status:
 *
 *  Succefully sorted out the database and create user
 *  function. Next step is to work on mounting a login
 *  handeler that works with a form. I also need to
 *  implement the user::exists function.
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

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("gift_list")]
pub struct Db(sqlx::SqlitePool);

#[get("/")]
async fn index(mut db: Connection<Db>) -> &'static str {
    match user::User::new(db, "John Doe", &None, "Password123").await {
        Ok(_) => "User Succefully created",
        Err(_) => "An error occured",
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index])
}
