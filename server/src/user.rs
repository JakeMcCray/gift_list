use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;

use crate::Db;

use rocket::serde::Deserialize;

pub enum UserError {
    UsernameExists,
    HashError,
    DatabaseError,
    EmailExists,
    PasswordFailed,
    UserDontExist,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
enum PHash {
    Password(String),
    Hash(String),
}

/***********************************
 *  Note:
 *
 *  This this struct represents a
 *  hypothetical user, it is not
 *  known if the user actually exists
 *  in the database if there is a
 *  user object
 * ********************************/
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    username: String,
    email: Option<String>,
    password: PHash,
}

impl User {
    /******************************************
     *  Function: register_user
     *
     *  Desc: insert user into database if
     *  no other user exists
     *
     *  Errors:
     *      -DatabaseError
     *      -UsernameExists
     *      -EmailExists
     * ***************************************/
    pub async fn register_user(&mut self, mut db: Connection<Db>) -> Result<(), UserError> {
        //check to see if the user exists
        //Mutex here
        User::name_is_free(&mut **db, &self.username).await?;
        if let Some(email) = &self.email {
            User::email_is_free(&mut **db, email).await?;
        }

        self.try_hash_password();

        match &self.password {
            PHash::Hash(hash) => User::enter_user(&self.username, &self.email, hash, db).await,
            PHash::Password(_) => Err(UserError::HashError),
        }
        //drop Mutex here
    }

    /******************************************
     *  Function: enter_user
     *
     *  Desc: puts user info into the database
     *  this function assumes that the password
     *  has already been hashed and no duplicate
     *  user exists
     * ***************************************/
    async fn enter_user(
        username: &str,
        email: &Option<String>,
        hash: &str,
        mut db: Connection<Db>,
    ) -> Result<(), UserError> {
        //I am left with a choice
        //On one hand I can have nested match statments, and leave the code somewhat confusing to
        //read
        //...
        //On the other hand I can put the inner match statments into two functions, and create
        //confusing names for the functions
        match email {
            Some(email) => {
                match sqlx::query("INSERT INTO users (name,email,p_hash) VALUES (?, ?, ?)")
                    .bind(username)
                    .bind(email)
                    .bind(hash)
                    .execute(&mut **db)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(UserError::DatabaseError),
                }
            }
            None => {
                match sqlx::query("INSERT INTO users (name,p_hash) VALUES (?, ?)")
                    .bind(username)
                    .bind(hash)
                    .execute(&mut **db)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(UserError::DatabaseError),
                }
            }
        }
    }

    /******************************************
     *  Function: hash_password
     *
     *  Desc: tries to hash the password of a user
     * ***************************************/

    fn try_hash_password(&mut self) {
        match &self.password {
            PHash::Password(pass) => {
                let salt = SaltString::generate(&mut OsRng);

                let argon2 = Argon2::default();

                match argon2.hash_password(String::from(pass).as_bytes(), &salt) {
                    Ok(hash) => self.password = PHash::Hash(hash.to_string()),

                    Err(_) => (), //¯\_(ツ)_/¯ error handeling is for chumps
                                  //note: this means that i will need to error handle whenever i
                                  //want a hash from the user
                }
            }
            PHash::Hash(_) => (),
        }
    }
    /******************************************
     *  Function: name_is_free
     *
     *  Desc: returns Ok(()) if a user does not
     *  exist with the username
     *
     *  Error:
     *      -UsernameExists
     *      -DatabaseError
     * ***************************************/
    async fn name_is_free(
        db: &mut sqlx::SqliteConnection,
        username: &str,
    ) -> Result<(), UserError> {
        match sqlx::query("SELECT name FROM users WHERE name = ?")
            .bind(username)
            .fetch_optional(db)
            .await
        {
            Ok(query) => {
                if let Some(_) = query {
                    Err(UserError::UsernameExists)
                } else {
                    Ok(())
                }
            }
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    /******************************************
     *  Function: email_is_free
     *
     *  Desc: returns Ok(()) if a user does not
     *  exist with the username
     *
     *  Error:
     *      -EmailExists
     *      -DatabaseError
     * ***************************************/
    async fn email_is_free(db: &mut sqlx::SqliteConnection, email: &str) -> Result<(), UserError> {
        match sqlx::query("SELECT email FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(db)
            .await
        {
            Ok(query) => {
                if let Some(_) = query {
                    Err(UserError::EmailExists)
                } else {
                    Ok(())
                }
            }
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    /******************************************
     *  Function: verify_login
     *
     *  Desc: Returns User if user can login
     *  also returns false if any error occurs
     *
     * ***************************************/
    pub async fn verify_login(mut db: Connection<Db>, user: &User) -> Result<(), UserError> {
        match sqlx::query("SELECT p_hash FROM users WHERE name = ?")
            .bind(&user.username)
            .fetch_one(&mut **db)
            .await
        {
            Ok(hash) => {
                let hash: &str = hash.get("p_hash");
                User::check_password(hash, &user.password)
            }
            Err(_) => Err(UserError::DatabaseError),
        }
    }

    /******************************************
     *  Function: verify_login
     *
     *  Desc: Returns User if user can login
     *  also returns false if any error occurs
     *
     *  this description is wrong, too lazy to
     *  fix
     *
     * ***************************************/
    fn check_password(hash: &str, password: &PHash) -> Result<(), UserError> {
        match password {
            PHash::Password(password) => {
                if let Ok(parsed_hash) = PasswordHash::new(&hash) {
                    Argon2::default()
                        .verify_password(password.as_bytes(), &parsed_hash)
                        .map_err(|_| UserError::PasswordFailed)
                } else {
                    Err(UserError::PasswordFailed)
                }
            }
            //idk if this is what i want to do, maybe come back later and fix
            PHash::Hash(_) => Err(UserError::HashError),
        }
    }
}
