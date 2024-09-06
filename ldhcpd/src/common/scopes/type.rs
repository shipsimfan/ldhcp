use router::sql::{Bind, Statement};

/// The type of item a scope is for
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeType {
    /// This scope applies to all clients
    Global = 1,

    /// This scope applies to a single reservation
    Reservation = 2,
}

impl Bind for ScopeType {
    fn bind<'a, S: Statement<'a>>(
        &'a self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError> {
        statement.bind_usize(idx, *self as usize)
    }
}
