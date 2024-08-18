use data_format::{Deserialize, DeserializeError, Serialize};
use sql::FromColumn;

/// An identifier for a connecting client
#[derive(Debug, PartialEq, Eq)]
pub struct ClientID(Vec<u8>);

/// The maximum length of a [`ClientID`]
const MAX_CLIENT_ID_LENGTH: usize = 256;

impl ClientID {
    /// Gets the client id as a slice
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Serialize for ClientID {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ClientID {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let client_id = Vec::<u8>::deserialize(deserializer)?;

        if client_id.len() > MAX_CLIENT_ID_LENGTH {
            Err(D::Error::invalid_length(client_id.len(), "less than 256"))
        } else if client_id.len() == 0 {
            Err(D::Error::invalid_length(client_id.len(), "non-zero"))
        } else {
            Ok(ClientID(client_id))
        }
    }
}

impl FromColumn for ClientID {
    fn from_column<'a, C: sql::Column<'a>>(column: C) -> Result<Self, C::Error> {
        column.into_blob().map(|blob| ClientID(blob.to_vec()))
    }
}
