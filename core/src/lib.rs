pub mod app_setup;
pub mod app_state;
pub mod auth;
pub mod entities;
pub mod enums;
pub mod helpers;
pub mod http;
pub mod models;
pub mod permissions;
pub mod repositories;
pub mod results;
pub mod roles;
pub mod schema;
pub mod services;
pub mod user_setup;
pub mod uuid;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
