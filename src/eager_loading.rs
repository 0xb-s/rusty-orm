use crate::{model::Model, query_builder::SelectQuery};

/// Represents a relationship between two models.
pub trait Relationship {
    type RelatedModel: Model;

    /// The foreign key in the current model.
    fn foreign_key() -> String;

    /// The primary key in the related model.
    fn related_key() -> String;
}

///  eager loading of related entities.
pub struct EagerLoader<T: Model> {
    base_query: SelectQuery<T>,
}

impl<T: Model> EagerLoader<T> {
    /// Creates a new EagerLoader with the provided base query.
    pub fn new(base_query: SelectQuery<T>) -> Self {
        EagerLoader { base_query }
    }

    /// Adds a join for the specified relationship.
    pub fn with<R: Relationship>(mut self) -> Self {
        let base_table = &self.base_query.table;
        let related_table = R::RelatedModel::table();

        let join_condition = format!(
            "{}.{} = {}.{}",
            base_table.name,
            R::foreign_key(),
            related_table.name,
            R::related_key()
        );

        let join_clause = format!("INNER JOIN {} ON {}", related_table.name, join_condition);
        self.base_query.joins.push(join_clause);

        self
    }

    /// Builds the final SQL query string with joins.
    pub fn build(self) -> String {
        self.base_query.build()
    }
}
