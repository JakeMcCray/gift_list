use argon2::Argon2;

enum UserError {
    UsernameExists,
    EmailExists,
}

struct User {
    username: String,
    email: Option<String>,
    p_hash: String,
}

impl User {
    fn new(username: &str, email: &Option<String>, password: &str) -> Result<User, UserError> {
        User::exists(username, email)?;
        //hash password
        //add user details to database
        Ok(User {
            username: String::from(username),
            email: email.clone(),
            p_hash: String::from(password),
        })
    }

    fn exists(username: &str, email: &Option<String>) -> Result<(), UserError> {
        Err(UserError::UsernameExists)
    }
}
