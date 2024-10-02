use crate::model::Model;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

/// Represents a migration with up and down SQL statements.
#[derive(Debug, Serialize, Deserialize)]
pub struct Migration {
    pub up: String,
    pub down: String,
}

/// Generates a migration based on current and previous schemas.
pub struct MigrationGenerator;

impl MigrationGenerator {
    /// Generates a migration by comparing current and previous tables.
    ///
    /// 
    /// TODO: ameliorate this
    pub fn generate<T: Model>() -> Migration {
        let table = T::table();

        // TODO BETTER
      // Generate simple CREATE TABLE and DROP TABLE statements
        let up = format!(
            "CREATE TABLE {} ({});",
            table.name,
            table
                .columns
                .iter()
                .map(|col| format!(
                    "{} {}{}",
                    col.name,
                    map_data_type_to_sql(&col.data_type),
                    if col.is_primary_key { " PRIMARY KEY" } else { "" }
                ))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let down = format!("DROP TABLE IF EXISTS {};", table.name);

        Migration { up, down }
    }

    /// Saves the migration to the specified directory with the given name.
    pub fn save_migration(migration: &Migration, name: &str, path: &str) -> std::io::Result<()> {
        let migration_dir = Path::new(path);
        fs::create_dir_all(migration_dir)?;
        let migration_file = migration_dir.join(format!("{}.json", name));
        let serialized = serde_json::to_string_pretty(migration)?;
        let mut file = File::create(migration_file)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

/// Maps the ORM's DataType to actual SQL data types.
fn map_data_type_to_sql(data_type: &crate::model::DataType) -> String {
    match data_type {
        crate::model::DataType::Integer => "INTEGER".to_string(),
        crate::model::DataType::Varchar(size) => format!("VARCHAR({})", size),
        crate::model::DataType::Boolean => "BOOLEAN".to_string(),
        crate::model::DataType::Float => "FLOAT".to_string(),
    }
}
