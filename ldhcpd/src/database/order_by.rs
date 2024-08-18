/// How to order the results of a query
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderByDirection {
    /// Order the results in ascending order
    #[default]
    Ascending,

    /// Order the results in descending order
    Descending,
}

/// A field and direction to order by
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderBy<Field> {
    direction: OrderByDirection,
    field: Field,
}
