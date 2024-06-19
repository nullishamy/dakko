use serde::ser::{Serialize, Serializer};
pub struct DakkoError(megalodon::error::Error);

impl Serialize for DakkoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl From<megalodon::error::Error> for DakkoError {
    fn from(value: megalodon::error::Error) -> Self {
        Self(value)
    }
}
