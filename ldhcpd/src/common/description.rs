use router::{
    data_format::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer},
    sql::{Bind, Column, FromColumn},
};
use std::borrow::Cow;

/// A description of an element
#[derive(Debug)]
pub struct Description<'a>(Cow<'a, str>);

/// The maximum length of a [`Description`]
const MAX_DESCRIPTION_LENGTH: usize = 4096;

impl<'a> Description<'a> {
    /// Gets the description as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Description<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let description = Cow::<'de, str>::deserialize(deserializer)?;
        if description.len() > MAX_DESCRIPTION_LENGTH {
            return Err(D::Error::invalid_length(description.len(), "4096"));
        }

        Ok(Description(description))
    }
}

impl<'a> Serialize for Description<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_string(&self.0)
    }
}

impl FromColumn for Description<'static> {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<Self, C::Error> {
        column
            .into_str()
            .map(|description| Description(Cow::Owned(description.into())))
    }
}

impl<'a> Bind for Description<'a> {
    fn bind<'b, S: router::sql::Statement<'b>>(
        &'b self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_str(idx, &self.0)
    }
}
