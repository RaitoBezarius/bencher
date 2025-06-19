use derive_more::Display;
use email_address::EmailAddress;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use std::{fmt, str::FromStr};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, Visitor},
};

use crate::{Sanitize, ValidError, secret::SANITIZED_SECRET};

#[typeshare::typeshare]
#[derive(Debug, Display, Clone, Eq, PartialEq, Hash, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "db", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(feature = "db", diesel(sql_type = diesel::sql_types::Text))]
pub struct Email(String);

#[cfg(feature = "db")]
crate::typed_string!(Email);

impl FromStr for Email {
    type Err = ValidError;

    fn from_str(email: &str) -> Result<Self, Self::Err> {
        if is_valid_email(email) {
            Ok(Self(email.to_lowercase()))
        } else {
            Err(ValidError::Email(email.into()))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl Sanitize for Email {
    fn sanitize(&mut self) {
        self.0 = SANITIZED_SECRET.into();
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(EmailVisitor)
    }
}

struct EmailVisitor;

impl Visitor<'_> for EmailVisitor {
    type Value = Email;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid email")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse().map_err(E::custom)
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn is_valid_email(email: &str) -> bool {
    EmailAddress::is_valid(email)
}

#[cfg(test)]
mod test {
    use std::str::FromStr as _;

    use super::{Email, is_valid_email};
    use pretty_assertions::assert_eq;

    use crate::test::{LEN_64_STR, LEN_65_STR};

    #[test]
    fn test_email() {
        assert_eq!(true, is_valid_email("abc.xyz@example.com"));
        assert_eq!(true, is_valid_email("abc@example.com"));
        assert_eq!(true, is_valid_email("a@example.com"));
        assert_eq!(true, is_valid_email("abc.xyz@example.co"));
        assert_eq!(true, is_valid_email("abc@example.co"));
        assert_eq!(true, is_valid_email("a@example.co"));
        assert_eq!(true, is_valid_email("abc.xyz@example"));
        assert_eq!(true, is_valid_email("abc@example"));
        assert_eq!(true, is_valid_email("a@example"));
        assert_eq!(
            Email::from_str("abc.xyz@example.com").unwrap(),
            Email::from_str("ABC.xYz@Example.coM").unwrap()
        );
        assert_eq!(true, is_valid_email(&format!("{LEN_64_STR}@example.com")));

        assert_eq!(false, is_valid_email(""));
        assert_eq!(false, is_valid_email(" abc@example.com"));
        assert_eq!(false, is_valid_email("abc @example.com"));
        assert_eq!(false, is_valid_email("abc@example.com "));
        assert_eq!(false, is_valid_email("example.com"));
        assert_eq!(false, is_valid_email("abc.example.com"));
        assert_eq!(false, is_valid_email("abc!example.com"));
        assert_eq!(false, is_valid_email(&format!("{LEN_65_STR}@example.com")));
    }
}
