/// An unknown field name was provided in a query parameter
#[derive(Debug)]
pub struct UnknownField<'a>(&'a [u8]);

impl<'a> UnknownField<'a> {
    /// Creates a new [`UnknownField`] error
    pub(super) fn new(field: &'a [u8]) -> Self {
        UnknownField(field)
    }
}

impl<'a> std::error::Error for UnknownField<'a> {}

impl<'a> std::fmt::Display for UnknownField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown field \"{}\"", String::from_utf8_lossy(self.0))
    }
}
