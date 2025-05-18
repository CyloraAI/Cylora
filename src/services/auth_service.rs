pub fn validate_credentials(username: &str, password: &str) -> bool {
    username == "admin" && password == "password"
}
