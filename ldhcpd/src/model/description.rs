use data_format::{Deserialize, DeserializeError};
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
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let description = Cow::<'de, str>::deserialize(deserializer)?;
        if description.len() > MAX_DESCRIPTION_LENGTH {
            return Err(D::Error::invalid_length(description.len(), "4096"));
        }

        Ok(Description(description))
    }
}
