extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Type,
};

/// Procedural macro to derive the `Model` trait for a struct.
///
/// Usage:
/// ```rust
/// #[derive(Model)]
/// #[table_name = "users"] // Optional: specify table name
/// struct User {
///     #[column(type = "Integer", primary_key = "true")]
///     id: i32,
///     #[column(type = "Varchar(100)")]
///     name: String,
///     email: String, // Defaults to Varchar(255)
/// }
/// ```
#[proc_macro_derive(Model, attributes(table_name, column))]
pub fn derive_model(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);


    let name = input.ident.clone();


    let table_name = match get_table_name(&input) {
        Some(name) => name,
        None => name.to_string().to_lowercase(),
    };


    let columns = match get_columns(&input) {
        Ok(cols) => cols,
        Err(e) => return e.to_compile_error().into(),
    };


    let column_defs = columns.iter().map(|col| {
        let col_name = &col.name;
        let data_type = &col.data_type;
        let is_pk = col.is_primary_key;
        quote! {
            Column {
                name: #col_name.to_string(),
                data_type: #data_type,
                is_primary_key: #is_pk,
            },
        }
    });

    // Generate the implementation of the Model trait
    let expanded = quote! {
        impl Model for #name {
            fn table() -> Table {
                Table {
                    name: #table_name.to_string(),
                    columns: vec![
                        #(#column_defs)*
                    ],
                }
            }
        }
    };

    // Convert into a TokenStream and return
    TokenStream::from(expanded)
}

/// Extracts the table name from the struct attributes.
fn get_table_name(input: &DeriveInput) -> Option<String> {
    for attr in &input.attrs {
        if let Ok(meta) = attr.parse_meta() {
            if let Meta::NameValue(MetaNameValue { path, lit, .. }) = meta {
                if path.is_ident("table_name") {
                    if let Lit::Str(lit_str) = lit {
                        return Some(lit_str.value());
                    }
                }
            }
        }
    }
    None
}

/// Represents a column during macro processing.
struct ColumnInfo {
    name: String,
    data_type: proc_macro2::TokenStream,
    is_primary_key: bool,
}

/// Extracts column information from the struct fields.
fn get_columns(input: &DeriveInput) -> Result<Vec<ColumnInfo>, syn::Error> {
    let mut columns = Vec::new();

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Err(syn::Error::new_spanned(input, "Model can only be derived for structs"));
        }
    };

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap().to_string();

        // Default data type based on Rust type
        let default_data_type = map_rust_type_to_sql(&field.ty);

        let mut data_type = default_data_type.clone();
        let mut is_primary_key = false;

        // Check for custom column attributes
        for attr in &field.attrs {
            if attr.path.is_ident("column") {
                let meta = attr.parse_meta()?;
                if let Meta::List(MetaList { nested, .. }) = meta {
                    for nested_meta in nested.iter() {
                        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                            path, lit, ..
                        })) = nested_meta
                        {
                            if path.is_ident("type") {
                                if let Lit::Str(lit_str) = lit {
                                    data_type = parse_sql_type(&lit_str.value())?;
                                }
                            } else if path.is_ident("primary_key") {
                                if let Lit::Str(lit_str) = lit {
                                    is_primary_key =
                                        lit_str.value().parse::<bool>().unwrap_or(false);
                                }
                            }
                        }
                    }
                }
            }
        }

        columns.push(ColumnInfo { name: field_name, data_type, is_primary_key });
    }

    Ok(columns)
}

/// Maps Rust types to SQL data types.
fn map_rust_type_to_sql(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let ident = &type_path.path.segments.last().unwrap().ident;
            match ident.to_string().as_str() {
                "i32" | "i64" => quote! { DataType::Integer },
                "String" => quote! { DataType::Varchar(255) },
                "bool" => quote! { DataType::Boolean },
                "f32" | "f64" => quote! { DataType::Float },
                _ => quote! { DataType::Varchar(255) }, //todo better here handling 
            }
        }
        _ => quote! { DataType::Varchar(255) },  //todo better here handling 
    }
}

/// Parses a custom SQL type from a string.
fn parse_sql_type(type_str: &str) -> Result<proc_macro2::TokenStream, syn::Error> {
    if type_str.starts_with("Varchar") {
    
        let start = type_str.find('(').ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected '(' in Varchar type definition",
            )
        })?;
        let end = type_str.find(')').ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected ')' in Varchar type definition",
            )
        })?;
        let size_str = &type_str[start + 1..end];
        let size: usize = size_str.parse().map_err(|_| {
            syn::Error::new(proc_macro2::Span::call_site(), "Failed to parse size for Varchar")
        })?;
        Ok(quote! { DataType::Varchar(#size) })
    } else {
        match type_str {
            "Integer" => Ok(quote! { DataType::Integer }),
            "Boolean" => Ok(quote! { DataType::Boolean }),
            "Float" => Ok(quote! { DataType::Float }),
            other => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Unsupported data type: {}", other),
            )),
        }
    }
}
