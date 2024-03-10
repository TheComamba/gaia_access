#[cfg(test)]
mod tests {
    use gaia_access::{column::GaiaColumn, error::GaiaError, schema::GaiaSchema, table::GaiaTable};
    use reqwest;
    use std::{collections::HashMap, fs, io::BufReader};
    use strum::IntoEnumIterator;
    use xmltree::Element;

    const FILE_PATH: &'static str = "dev_data/tables.xml";

    struct GaiaTables {
        schemas: Vec<String>,
        tables: Vec<String>,
        columns: Vec<String>,
    }

    type Tables = HashMap<String, Vec<String>>;
    type Schemas = HashMap<String, Tables>;

    fn check_and_download_file() -> Result<(), GaiaError> {
        if !fs::metadata(FILE_PATH).is_ok() {
            fs::create_dir_all("dev_data").map_err(|e| GaiaError::General(e.to_string()))?;

            let url = "https://gea.esac.esa.int/tap-server/tap/tables";
            println!("Downloading {}...", url);
            let mut response = reqwest::blocking::get(url)?;

            let mut file =
                fs::File::create(FILE_PATH).map_err(|e| GaiaError::General(e.to_string()))?;
            response.copy_to(&mut file)?;
        }
        Ok(())
    }

    fn read_xml_file() -> Result<Schemas, GaiaError> {
        check_and_download_file()?;
        let file = fs::File::open(FILE_PATH).map_err(|e| GaiaError::General(e.to_string()))?;

        let mut gaia_schemas = Schemas::new();

        let reader = BufReader::new(file);
        let root = Element::parse(reader).map_err(|e| GaiaError::General(e.to_string()))?;
        for schema in root.children {
            let schema = schema.as_element().unwrap();
            let schema_name = schema.get_child("name").unwrap().children[0]
                .as_text()
                .unwrap();
            let mut tables = Tables::new();
            for table in &schema.children {
                let table = table.as_element().unwrap();
                if table.name != "table" {
                    continue;
                }
                let table_name = table.get_child("name").unwrap().children[0]
                    .as_text()
                    .unwrap();
                let table_name_parts: Vec<&str> = table_name.split('.').collect();
                let table_name = table_name_parts.last().unwrap();
                let mut columns = Vec::new();
                for column in &table.children {
                    let column = column.as_element().unwrap();
                    if column.name != "column" {
                        continue;
                    }
                    let column_name = column.get_child("name").unwrap().children[0]
                        .as_text()
                        .unwrap();
                    columns.push(column_name.to_string());
                }
                tables.insert(table_name.to_string(), columns);
            }
            gaia_schemas.insert(schema_name.to_string(), tables.clone());
        }

        Ok(gaia_schemas)
    }

    #[test]
    #[ignore]
    fn print_all_tables() {
        let result = read_xml_file().unwrap();
        for (schema, tables) in result {
            println!("\nSchema: {}\n", schema);
            for (table, _) in tables {
                println!("{}", table);
            }
        }
    }

    #[test]
    #[ignore]
    fn print_all_columns() {
        let result = read_xml_file().unwrap();
        for (schema, tables) in result {
            println!("Schema: {}", schema);
            for (table, columns) in tables {
                println!("  Table: {}", table);
                for column in columns {
                    println!("    Column: {}", column);
                }
            }
        }
    }

    // #[test]
    // #[ignore]
    // fn all_rust_schemas_are_in_xml() {
    //     let result = read_xml_file().unwrap();
    //     let mut missing_schemas = result.schemas.clone();
    //     for schema in GaiaSchema::iter() {
    //         let schema = schema.to_string();
    //         assert!(result.schemas.contains(&schema));
    //         missing_schemas.retain(|s| s != &schema);
    //     }
    //     println!("Missing schemas: {:?}", missing_schemas);
    // }

    // #[test]
    // #[ignore]
    // fn all_rust_tables_are_in_xml() {
    //     let result = read_xml_file().unwrap();
    //     let mut missing_tables = result.tables.clone();
    //     for table in GaiaTable::iter() {
    //         let table = table.to_string();
    //         assert!(result.tables.contains(&table));
    //         missing_tables.retain(|t| t != &table);
    //     }
    //     println!("Missing tables: {:?}", missing_tables);
    // }

    // #[test]
    // #[ignore]
    // fn all_rust_columns_are_in_xml() {
    //     let result = read_xml_file().unwrap();
    //     let mut missing_columns = result.columns.clone();
    //     for column in GaiaColumn::iter() {
    //         let column = column.to_string();
    //         assert!(result.columns.contains(&column));
    //         missing_columns.retain(|c| c != &column);
    //     }
    //     println!("Missing columns: {:?}", missing_columns);
    // }
}
