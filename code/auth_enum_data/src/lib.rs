pub fn read_line() -> String {
    // <- Public function
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let username = username.to_lowercase();
    if username == "admin" && password == "password" {
        LoginAction::Granted(LoginRole::Admin)
    } else if username == "bob" && password == "password" {
        LoginAction::Granted(LoginRole::User)
    } else {
        LoginAction::Denied
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_login() {
        assert_eq!(login("admin", "password"), LoginAction::Granted(LoginRole::Admin));
        assert_eq!(login("bob", "password"), LoginAction::Granted(LoginRole::User));
        assert_eq!(login("bob", "wrong"), LoginAction::Denied);
    }
}
