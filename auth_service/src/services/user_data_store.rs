use std::collections::HashMap;
// use  crate::entities::{Email, Password, User};
use crate::domain::{Email, Password, User, UserStore, UserStoreError};
use async_trait::async_trait;


#[derive(Default)]
pub struct UserDataStore {
    users: HashMap<Email, User>
}

#[async_trait]
impl UserStore for UserDataStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound)
        }
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = UserDataStore::default();
        let user = User {
            email: Email::parse(String::from("johndoe@mail.com")).unwrap(), 
            password: Password::parse(String::from("plsdonthackme")).unwrap(), 
            requires_2fa: false
        };
        // Test adding a new user
        let result = store.add_user(user.clone()).await;
        assert!(result.is_ok());

        
        // Test adding an existing user
        let result = store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = UserDataStore::default();
        let email = Email::parse(String::from("johndoe@mail.com")).unwrap();
        let password = Password::parse(String::from("plsdonthackme")).unwrap();

        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false
        };

        // Test getting a user that exists
        store.users.insert(email.clone(), user.clone());
        let result = store.get_user(&email).await;
        assert_eq!(result, Ok(user));

        // Test getting a user that doesn't exist
        let result = store
            .get_user(&Email::parse(String::from("nonexistent@mail.com")).unwrap())
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = UserDataStore::default();
        let email = Email::parse(String::from("johndoe@mail.com")).unwrap();
        let password = Password::parse(String::from("plsdonthackme")).unwrap();

        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false
        };

        // Test validating a user that exists with correct password
        store.users.insert(email.clone(), user.clone());
        let result = store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        // Test validating a user that exists with incorrect password
        let wrong_password = Password::parse("wrongpassword".to_owned()).unwrap();
        let result = store.validate_user(&email, &wrong_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test validating a user that doesn't exist
        let result = store
            .validate_user(
                &Email::parse("nonexistent@example.com".to_string()).unwrap(),
                &password,
            )
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
