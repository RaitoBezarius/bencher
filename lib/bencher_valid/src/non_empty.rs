use derive_more::Display;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use std::{fmt, str::FromStr};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, Visitor},
};

use crate::{UserName, ValidError};

#[typeshare::typeshare]
#[derive(Debug, Display, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "db", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(feature = "db", diesel(sql_type = diesel::sql_types::Text))]
pub struct NonEmpty(String);

#[cfg(feature = "db")]
crate::typed_string!(NonEmpty);

impl FromStr for NonEmpty {
    type Err = ValidError;

    fn from_str(non_empty: &str) -> Result<Self, Self::Err> {
        if is_valid_non_empty(non_empty) {
            Ok(Self(non_empty.into()))
        } else {
            Err(ValidError::NonEmpty(non_empty.into()))
        }
    }
}

impl AsRef<str> for NonEmpty {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<NonEmpty> for String {
    fn from(non_empty: NonEmpty) -> Self {
        non_empty.0
    }
}

impl From<UserName> for NonEmpty {
    fn from(user_name: UserName) -> Self {
        Self(user_name.into())
    }
}

impl<'de> Deserialize<'de> for NonEmpty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NonEmptyVisitor)
    }
}

struct NonEmptyVisitor;

impl Visitor<'_> for NonEmptyVisitor {
    type Value = NonEmpty;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a non-empty string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse().map_err(E::custom)
    }
}

impl NonEmpty {
    pub const MAX_LEN: usize = crate::MAX_LEN;
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn is_valid_non_empty(non_empty: &str) -> bool {
    crate::is_valid_non_empty(non_empty)
}

#[cfg(test)]
mod test {
    use crate::test::{LEN_0_STR, LEN_64_STR, LEN_65_STR};

    use super::is_valid_non_empty;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_non_empty() {
        assert_eq!(true, is_valid_non_empty("a"));
        assert_eq!(true, is_valid_non_empty("ab"));
        assert_eq!(true, is_valid_non_empty("abc"));
        assert_eq!(true, is_valid_non_empty("ABC"));
        assert_eq!(true, is_valid_non_empty("abc ~ABC!"));
        assert_eq!(true, is_valid_non_empty(LEN_64_STR));
        assert_eq!(true, is_valid_non_empty(LEN_65_STR));

        assert_eq!(false, is_valid_non_empty(LEN_0_STR));
    }
}
