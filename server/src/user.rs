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
    password: String,
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
        };

        let hash = Self::try_hash_password(self.password.to_owned())?;

        Self::enter_user(&self.username, &self.email, &hash, db).await?;
        //drop Mutex here
        Ok(())
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
        match email {
            Some(email) => {
                sqlx::query("INSERT INTO users ('name', 'email', 'pHash') VALUES (?, ?, ?)")
                    .bind(username)
                    .bind(email)
                    .bind(hash)
                    .execute(&mut **db)
                    .await
                    .map(|_| ())
                    .map_err(|_| UserError::DatabaseError)
            }
            None => sqlx::query("INSERT INTO users ('name', 'pHash') VALUES (?, ?)")
                .bind(username)
                .bind(hash)
                .execute(&mut **db)
                .await
                .map(|_| ())
                .map_err(|_| UserError::DatabaseError),
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
     *  Desc: Returns UserID if user can login
     *  also returns false if any error occurs
     *
     * ***************************************/
    pub async fn verify_login(mut db: Connection<Db>, user: &User) -> Result<String, UserError> {
        let hash: String = sqlx::query("SELECT pHash FROM users WHERE name = ?")
            .bind(&user.username)
            .fetch_one(&mut **db)
            .await
            .map_err(|_| UserError::DatabaseError)?
            .get("pHash");

        let parsed_hash = PasswordHash::new(&hash).map_err(|_| UserError::HashError)?;
        if Argon2::default()
            .verify_password(user.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            user.get_id(db).await
        } else {
            Err(UserError::PasswordFailed)
        }
    }

    /******************************************
     *  Function: get_id
     *
     *  Desc: returns the userID of the user
     * ***************************************/
    async fn get_id(&self, mut db: Connection<Db>) -> Result<String, UserError> {
        let id: u32 = sqlx::query("SELECT UserID FROM users WHERE name = ?")
            .bind(&self.username)
            .fetch_one(&mut **db)
            .await
            .map_err(|_| UserError::DatabaseError)?
            .get("UserID");
        Ok(id.to_string())
    }

    /******************************************
     *  Function: try_hash_password
     *
     *  Desc: tries to hash the password of a user
     *  and returns the hashed password
     * ***************************************/

    fn try_hash_password(password: String) -> Result<String, UserError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        argon2
            .hash_password(String::from(password).as_bytes(), &salt)
            .map(|a| a.to_string())
            .map_err(|_| UserError::HashError)
    }
}
