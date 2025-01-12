use crate::models::user::User;

pub struct UserService;

impl UserService {
    pub fn get_all_users() -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "Admin User 1".to_string(),
                email: "admin1@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Admin User 2".to_string(),
                email: "admin2@example.com".to_string(),
            },
        ]
    }
}
