use dotenvy::dotenv;

pub fn get_env_var(key: &str, default: &str) -> String {
    dotenvy::var(key).unwrap_or_else(|_| default.to_string())
}
