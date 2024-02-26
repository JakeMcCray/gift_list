use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

use crate::Db;

pub enum UserError {
    UsernameExists,
    EmailExists,
    HashError,
    DatabaseError,
}

pub struct User {
    username: String,
    email: Option<String>,
    p_hash: String,
}

impl User {
    pub async fn new(
        mut db: Connection<Db>,
        username: &str,
        email: &Option<String>,
        password: &str,
    ) -> Result<User, UserError> {
        User::exists(username, email)?;
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

    fn exists(username: &str, email: &Option<String>) -> Result<(), UserError> {
        Ok(())
    }
}
