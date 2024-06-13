use serde::Deserialize;
use validator::{ValidateEmail, ValidateLength};

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Email, String> {
        if ValidateEmail::validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(format!("{} is not a valid email.", email))
        }
    }
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Password, String> {
        if ValidateLength::validate_length(&password, Some(8), Some(256), None) {
            Ok(Password(password))
        } else {
            Err(String::from("Failed to parse string to a Password type"))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::Password;

    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        assert!(Password::parse(String::new()).is_err());
    }

    #[test]
    fn string_less_than_8_characters_is_rejected() {
        assert!(Password::parse(String::from("1234567")).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..30).fake_with_rng(g);
            Self(password)
        }
    }
    
    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(valid_password.0).is_ok()
    }
}
