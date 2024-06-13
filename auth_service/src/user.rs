pub mod credentials;
pub mod store;

pub use credentials::{Email, Password};


#[derive(Clone, PartialEq, Debug)]
pub struct User {
    pub requires_2fa: bool,
    pub email: Email,
    pub password: Password,
}
