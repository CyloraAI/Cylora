pub fn create_jwt(user_id: i32) -> String {
    format!("token_for_user_{}", user_id)
}
