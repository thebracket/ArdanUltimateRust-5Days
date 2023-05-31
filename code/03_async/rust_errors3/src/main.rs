use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
fn load_users() -> GenericResult<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}

#[allow(dead_code)]
fn anyhow_load_users() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}

#[allow(dead_code)]
fn anyhow_load_users2() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    if users.is_empty() {
        anyhow::bail!("No users found");
    }
    if users.len() > 10 {
        return Err(anyhow::Error::msg("Too many users"));
    }
    Ok(users)
}

/*
// Do it the hard way

#[derive(Debug, Clone)]
enum UsersError {
    NoUsers, TooManyUsers
}

use std::fmt;

impl fmt::Display for UsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UsersError::NoUsers => write!(f, "no users found"),
            UsersError::TooManyUsers => write!(f, "too many users found"),
        }
    }
}
*/

// Do it the `thiserror` way:
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users were found")]
    TooManyUsers,
    #[error("Unable to open users file")]
    FileError,
    #[error("Unable to deserialize json")]
    JsonError(serde_json::Error),
}

#[allow(dead_code)]
fn work_with_my_error() -> Result<Vec<User>, UsersError> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file).map_err(|_| UsersError::FileError)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(UsersError::JsonError)?;
    if users.is_empty() {
        Err(UsersError::NoUsers)
    } else if users.len() > 10 {
        Err(UsersError::TooManyUsers)
    } else {
        Ok(users)
    }
}

fn main() {
    let users = anyhow_load_users();
    match users {
        Ok(users) => {
            for user in users {
                println!("User: {}, {}", user.name, user.password);
            }
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}
