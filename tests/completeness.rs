#[cfg(test)]
mod tests {
    use gaia_access::error::GaiaError;
    use reqwest;
    use std::{fs, io::BufReader};
    use xmltree::Element;

    const FILE_PATH: &'static str = "dev_data/tables.xml";

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

    fn read_xml_file() -> Result<(), GaiaError> {
        check_and_download_file()?;
        let file = fs::File::open(FILE_PATH).map_err(|e| GaiaError::General(e.to_string()))?;

        let reader = BufReader::new(file);
        let root = Element::parse(reader).map_err(|e| GaiaError::General(e.to_string()))?;
        println!("Root has {} children", root.children.len());
        for schema in root.children {
            let schema = schema.as_element().unwrap();
            println!(
                "Schema {} has {} children",
                schema.name,
                schema.children.len()
            );
        }

        Ok(())
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
