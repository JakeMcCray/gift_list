use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

use crate::Db;

pub enum UserError {
    UsernameExists,
    HashError,
    DatabaseError,
    EmailExists,
    PasswordFailed,
}

pub struct User {
    //id: u64,
    username: String,
    email: Option<String>,
    p_hash: String,
}

impl User {
    /******************************************
     *  Function: new
     *
     *  Desc: creates a new user and inserts
     *  them into the database
     *
     *  Error: returns an error if it is unable
     *  to create a user
     * ***************************************/
    pub async fn new(
        mut db: Connection<Db>,
        username: &str,
        email: &Option<String>,
        password: &str,
    ) -> Result<User, UserError> {
        //gives error if there is a conficting user
        User::name_is_free(&mut **db, username).await?;
        if let Some(email) = email {
            User::email_is_free(&mut **db, email).await?;
        }
        //hash password
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt);

        if let Ok(p_hash) = password_hash {
            //add user details to database
            if let Err(_) = match email {
                Some(email) => {
                    sqlx::query("INSERT INTO users (name, email, pHash) VALUES (?, ?, ?)")
                        .bind(username)
                        .bind(email)
                        .bind(p_hash.to_string())
                        .execute(&mut **db)
                        .await
                }
                None => {
                    sqlx::query("INSERT INTO users (name, pHash) VALUES (?, ?)")
                        .bind(username)
                        .bind(p_hash.to_string())
                        .execute(&mut **db)
                        .await
                }
            } {
                Err(UserError::DatabaseError)?;
            }
            //create user to return
            Ok(User {
                username: String::from(username),
                email: email.clone(),
                p_hash: p_hash.to_string(),
            })
        } else {
            Err(UserError::HashError)
        }
    }

    /******************************************
     *  Function: name_is_free
     *
     *  Desc: returns Ok(()) if a user does not
     *  exist with the username
     *
     *  Error: returns an error if there user
     *  exists or if cannot connect to database
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
     *  Error: returns an error if there user
     *  exists or if cannot connect to database
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
     *
     *  Error: Returns error if cannot find
     *  user or if password does not match
     * ***************************************/
    pub fn verify_login(user: User, password: &str) -> Result<User, UserError> {
        //brain no thinky, come back and do later
        todo!()
    }
}
