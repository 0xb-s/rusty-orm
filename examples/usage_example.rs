use rusty_orm::{
    migration::MigrationGenerator,
    model::{Column, DataType, Model, Table},
    query_builder::{DeleteQuery, InsertQuery, SelectQuery, UpdateQuery},
};
use rusty_orm_macros::Model;

#[derive(Model)]
#[table_name = "users"]
struct User {
    #[column(type = "Integer", primary_key = "true")]
    id: i32,
    #[column(type = "Varchar(100)")]
    name: String,
    #[column(type = "Varchar(150)")]
    email: String,
    is_active: bool, 
}

fn main() {
    let user_table = User::table();

    println!("Table Name: {}", user_table.name);
    println!("Columns:");
    for column in user_table.columns {
        println!(
            "- {}: {:?} {}",
            column.name,
            column.data_type,
            if column.is_primary_key { "(Primary Key)" } else { "" }
        );
    }


    let select_query = SelectQuery::<User>::new()
        .select(&["id", "name", "email"])
        .filter("is_active = true")
        .order_by(&["name"])
        .limit(10)
        .build();

    println!("\nGenerated SELECT Query:\n{}", select_query);

  
    let insert_query = InsertQuery::<User>::new()
        .value("name", "Alice")
        .value("email", "alice@example.com")
        .value("is_active", "true")
        .build();

    println!("\nGenerated INSERT Query:\n{}", insert_query);


    let update_query =
        UpdateQuery::<User>::new().set("email", "alice_new@example.com").filter("id = 1").build();

    println!("\nGenerated UPDATE Query:\n{}", update_query);


    let delete_query = DeleteQuery::<User>::new().filter("id = 1").build();

    println!("\nGenerated DELETE Query:\n{}", delete_query);


    let migration = MigrationGenerator::generate::<User>();
    match MigrationGenerator::save_migration(&migration, "create_users_table", "./migrations") {
        Ok(_) => println!("\nMigration 'create_users_table' saved successfully."),
        Err(e) => println!("\nFailed to save migration: {}", e),
    }
}
