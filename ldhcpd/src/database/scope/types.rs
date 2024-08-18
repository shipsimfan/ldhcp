/// A type that a scope can have
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeType {
    /// The global scope
    Global = 1,

    /// The scope is for a reservation
    Reservation = 2,
}

impl std::fmt::Display for ScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ScopeType::Global => "global",
            ScopeType::Reservation => "reservation",
        })
    }
}
