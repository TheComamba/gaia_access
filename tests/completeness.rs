#[cfg(test)]
mod tests {
    use gaia_access::{column::GaiaColumn, error::GaiaError, schema::GaiaSchema, table::GaiaTable};
    use reqwest;
    use std::{fs, io::BufReader};
    use strum::IntoEnumIterator;
    use xmltree::Element;

    const FILE_PATH: &'static str = "dev_data/tables.xml";

    struct GaiaTables {
        schemas: Vec<String>,
        tables: Vec<String>,
        columns: Vec<String>,
    }

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

    fn read_xml_file() -> Result<GaiaTables, GaiaError> {
        check_and_download_file()?;
        let file = fs::File::open(FILE_PATH).map_err(|e| GaiaError::General(e.to_string()))?;

        let mut gaia_tables = GaiaTables {
            schemas: Vec::new(),
            tables: Vec::new(),
            columns: Vec::new(),
        };

        let reader = BufReader::new(file);
        let root = Element::parse(reader).map_err(|e| GaiaError::General(e.to_string()))?;
        for schema in root.children {
            let schema = schema.as_element().unwrap();
            let schema_name = schema.get_child("name").unwrap().children[0]
                .as_text()
                .unwrap();
            gaia_tables.schemas.push(schema_name.to_string());
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
                gaia_tables.tables.push(table_name.to_string());
                for column in &table.children {
                    let column = column.as_element().unwrap();
                    if column.name != "column" {
                        continue;
                    }
                    let column_name = column.get_child("name").unwrap().children[0]
                        .as_text()
                        .unwrap();
                    gaia_tables.columns.push(column_name.to_string());
                }
            }
        }

        gaia_tables.schemas.sort();
        gaia_tables.schemas.dedup();
        gaia_tables.tables.sort();
        gaia_tables.tables.dedup();
        gaia_tables.columns.sort();
        gaia_tables.columns.dedup();

        println!("Schemas: {:?}", gaia_tables.schemas.len());
        println!("Tables: {:?}", gaia_tables.tables.len());
        println!("Columns: {:?}", gaia_tables.columns.len());

        Ok(gaia_tables)
    }

    #[test]
    fn all_rust_schemas_are_in_xml() {
        let result = read_xml_file().unwrap();
        let mut missing_schemas = result.schemas.clone();
        for schema in GaiaSchema::iter() {
            let schema = schema.to_string();
            assert!(result.schemas.contains(&schema));
            missing_schemas.retain(|s| s != &schema);
        }
        println!("Missing schemas: {:?}", missing_schemas);
    }

    #[test]
    fn all_rust_tables_are_in_xml() {
        let result = read_xml_file().unwrap();
        let mut missing_tables = result.tables.clone();
        for table in GaiaTable::iter() {
            let table = table.to_string();
            assert!(result.tables.contains(&table));
            missing_tables.retain(|t| t != &table);
        }
        println!("Missing tables: {:?}", missing_tables);
    }

    #[test]
    fn all_rust_columns_are_in_xml() {
        let result = read_xml_file().unwrap();
        let mut missing_columns = result.columns.clone();
        for column in GaiaColumn::iter() {
            let column = column.to_string();
            assert!(result.columns.contains(&column));
            missing_columns.retain(|c| c != &column);
        }
        println!("Missing columns: {:?}", missing_columns);
    }
}
