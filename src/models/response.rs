use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub timestamp: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    #[serde(flatten)]
    pub base: ApiResponse,
    pub user: super::user::User,
}
