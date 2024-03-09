#[cfg(test)]
mod tests {
    use gaia_access::error::GaiaError;
    use reqwest;
    use std::{fs, io::BufReader};
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
    #[ignore]
    fn test_check_and_download_file() {
        let result = check_and_download_file();
        assert!(result.is_ok(), "Error: {:?}", result.err());
    }

    #[test]
    #[ignore]
    fn test_read_xml_file() {
        let result = read_xml_file();
        assert!(result.is_ok(), "Error: {:?}", result.err());
    }
}
