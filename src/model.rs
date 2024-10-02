use serde::{Deserialize, Serialize};

/// Represents a column in a database table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub is_primary_key: bool,
}

/// Enum for various SQL data types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Integer,
    Varchar(usize),
    Boolean,
    Float,
    // todo add more
}

/// Represents a database table.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

/// Trait for entities that can be mapped to a database table.
pub trait Model {
    /// Returns the table schema associated with the model.
    fn table() -> Table;
}
