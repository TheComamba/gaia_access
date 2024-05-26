#[cfg(test)]
mod tests {
    use crate::{data::collect_known_schemas, error::GaiaError};
    use reqwest;
    use std::{collections::HashMap, fs, io::BufReader};
    use xmltree::Element;

    const FILE_PATH: &'static str = "dev_data/tables.xml";

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
    fn all_rust_data_is_in_xml() {
        let xml_schemas = read_xml_file().unwrap();
        let rust_schemas = collect_known_schemas();
        for (schema, tables) in rust_schemas {
            assert!(
                xml_schemas.contains_key(&schema),
                "Schema {} not found in XML",
                schema
            );
            for (table, columns) in tables {
                assert!(
                    xml_schemas[&schema].contains_key(&table),
                    "Table {} not found in XML",
                    table
                );
                for column in columns {
                    assert!(
                        xml_schemas[&schema][&table].contains(&column),
                        "Column {} not found in XML",
                        column
                    );
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn all_xml_data_is_in_rust() {
        let rust_schemas = collect_known_schemas();
        let xml_schemas = read_xml_file().unwrap();
        let mut missing_schemas = Vec::new();
        for (schema, tables) in xml_schemas {
            if !rust_schemas.contains_key(&schema) {
                missing_schemas.push(schema);
                continue;
            }
            let mut missing_tables = Vec::new();
            for (table, columns) in tables {
                if !rust_schemas[&schema].contains_key(&table) {
                    missing_tables.push(table);
                    continue;
                }
                let mut missing_columns = Vec::new();
                for column in columns {
                    if !rust_schemas[&schema][&table].contains(&column) {
                        missing_columns.push(column);
                    }
                }
                assert!(
                    missing_columns.is_empty(),
                    "Table {} is missing these columns:\n{:?}",
                    table,
                    missing_columns
                );
            }
            assert!(
                missing_tables.is_empty(),
                "Schema {} is missing these tables:\n{:?}",
                schema,
                missing_tables,
            );
        }
        assert!(
            missing_schemas.is_empty(),
            "Missing schemas in Rust:\n{:?}",
            missing_schemas
        );
    }
}
