pub fn read_line() -> String {
    // <- Public function
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

/// The version of the login that is a simple username and password
pub fn login_simple(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_login_simple() {
        assert!(login_simple("admin", "password"));
        assert!(!login_simple("admin", "wrong"));
        assert!(!login_simple("wrong", "password"));
    }
}
