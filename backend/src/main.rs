/*****************************************************
 *  Status:
 *
 *  At this point i am just working on getting the db
 *  working (using the rocket library).
 *  After that I will work on creating an account
 * **************************************************/
#[macro_use]
extern crate rocket;

mod user;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
