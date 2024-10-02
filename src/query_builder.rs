use crate::model::{Model, Table};
use std::marker::PhantomData;

/// Represents a SQL SELECT query.
#[derive(Debug, Default)]
pub struct SelectQuery<T: Model> {
    pub table: Table,
    selected_columns: Vec<String>,
    where_clause: Option<String>,
    pub joins: Vec<String>,
    order_by: Vec<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    _marker: PhantomData<T>,
}

impl<T: Model> SelectQuery<T> {
    /// Creates a new SelectQuery for the given model.
    pub fn new() -> Self {
        SelectQuery {
            table: T::table(),
            selected_columns: Vec::new(),
            where_clause: None,
            joins: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            _marker: PhantomData,
        }
    }

    /// Specifies the columns to select.
    pub fn select(mut self, columns: &[&str]) -> Self {
        self.selected_columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Adds a WHERE clause.
    pub fn filter(mut self, condition: &str) -> Self {
        self.where_clause = Some(condition.to_string());
        self
    }

    /// Adds an ORDER BY clause.
    pub fn order_by(mut self, columns: &[&str]) -> Self {
        self.order_by = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Adds a LIMIT clause.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Adds an OFFSET clause.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Builds the final SQL query string.
    pub fn build(self) -> String {
        let mut query = String::new();

        // SELECT clause
        if self.selected_columns.is_empty() {
            query.push_str("SELECT *");
        } else {
            query.push_str("SELECT ");
            query.push_str(&self.selected_columns.join(", "));
        }

        // FROM clause
        query.push_str(&format!(" FROM {}", self.table.name));

        // WHERE clause
        if let Some(where_clause) = self.where_clause {
            query.push_str(&format!(" WHERE {}", where_clause));
        }

        // ORDER BY clause
        if !self.order_by.is_empty() {
            query.push_str(&format!(" ORDER BY {}", self.order_by.join(", ")));
        }

        // LIMIT clause
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // OFFSET clause
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        query
    }
}

/// Represents a SQL INSERT query.
pub struct InsertQuery<T: Model> {
    table: Table,
    values: Vec<(String, String)>,
    _marker: PhantomData<T>,
}

impl<T: Model> InsertQuery<T> {
    /// Creates a new InsertQuery for the given model.
    pub fn new() -> Self {
        InsertQuery { table: T::table(), values: Vec::new(), _marker: PhantomData }
    }

    /// Adds a column-value pair to the INSERT statement.
    pub fn value(mut self, column: &str, value: &str) -> Self {
        self.values.push((column.to_string(), value.to_string()));
        self
    }

    /// Builds the final SQL query string.
    pub fn build(self) -> String {
        let columns: Vec<String> = self.values.iter().map(|(col, _)| col.clone()).collect();
        let values: Vec<String> = self.values.iter().map(|(_, val)| format!("'{}'", val)).collect();

        format!(
            "INSERT INTO {} ({}) VALUES ({});",
            self.table.name,
            columns.join(", "),
            values.join(", ")
        )
    }
}

/// Represents a SQL UPDATE query.
pub struct UpdateQuery<T: Model> {
    table: Table,
    set_clauses: Vec<(String, String)>,
    where_clause: Option<String>,
    _marker: PhantomData<T>,
}

impl<T: Model> UpdateQuery<T> {
    /// Creates a new UpdateQuery for the given model.
    pub fn new() -> Self {
        UpdateQuery {
            table: T::table(),
            set_clauses: Vec::new(),
            where_clause: None,
            _marker: PhantomData,
        }
    }

    /// Adds a SET clause.
    pub fn set(mut self, column: &str, value: &str) -> Self {
        self.set_clauses.push((column.to_string(), value.to_string()));
        self
    }

    /// Adds a WHERE clause.
    pub fn filter(mut self, condition: &str) -> Self {
        self.where_clause = Some(condition.to_string());
        self
    }

    /// Builds the final SQL query string.
    pub fn build(self) -> String {
        let set_clause: Vec<String> =
            self.set_clauses.iter().map(|(col, val)| format!("{} = '{}'", col, val)).collect();

        let mut query = format!("UPDATE {} SET {}", self.table.name, set_clause.join(", "));

        if let Some(where_clause) = self.where_clause {
            query.push_str(&format!(" WHERE {}", where_clause));
        }

        query
    }
}

/// Represents a SQL DELETE query.
pub struct DeleteQuery<T: Model> {
    table: Table,
    where_clause: Option<String>,
    _marker: PhantomData<T>,
}

impl<T: Model> DeleteQuery<T> {
    /// Creates a new DeleteQuery for the given model.
    pub fn new() -> Self {
        DeleteQuery { table: T::table(), where_clause: None, _marker: PhantomData }
    }

    /// Adds a WHERE clause.
    pub fn filter(mut self, condition: &str) -> Self {
        self.where_clause = Some(condition.to_string());
        self
    }

    /// Builds the final SQL query string.
    pub fn build(self) -> String {
        let mut query = format!("DELETE FROM {}", self.table.name);

        if let Some(where_clause) = self.where_clause {
            query.push_str(&format!(" WHERE {}", where_clause));
        }

        query
    }
}
