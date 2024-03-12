import os
import shutil
import chardet
import requests
from xml.etree import ElementTree as ET

FILE_PATH = "dev_data/file.xml"

SCHEMA_TEMPLATE = """
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct {name};

impl Schema for {name} {{
    fn string(&self) -> String {{
        "{name}".to_string()
    }}
}}

{modules}

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {{
    let mut tables = std::collections::HashMap::new();
    {known_tables}
    map.insert({name}.string(), tables);
}}
"""

TABLE_TEMPLATE = """
use crate::column::Column
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct {name};

impl Schema for {name} {{
    fn string(&self) -> String {{
        "{name}".to_string()
    }}
}}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {{
    {columns}
}}

impl Column for Col {{}}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {{
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert({name}.string(), col_strings);
}}
"""

class GaiaError(Exception):
    pass

def check_and_download_file():
    if not os.path.exists(FILE_PATH):
        os.makedirs("dev_data", exist_ok=True)

        url = "https://gea.esac.esa.int/tap-server/tap/tables"
        print(f"Downloading {url}...")
        response = requests.get(url)

        if response.status_code != 200:
            raise GaiaError("Failed to download file")

        with open(FILE_PATH, 'wb', encoding='UTF-8') as file:
            file.write(response.content)

def read_xml_file():
    check_and_download_file()

    if not os.path.exists(FILE_PATH):
        raise GaiaError("File not found")

    # Detect the file encoding
    rawdata = open(FILE_PATH, 'rb').read()
    result = chardet.detect(rawdata)
    charenc = result['encoding']

    with open(FILE_PATH, 'r', encoding=charenc) as file:
        tree = ET.parse(file)
        root = tree.getroot()

        gaia_schemas = {}

        for schema in root:
            schema_name = schema.find("name").text
            tables = {}
            for table in schema:
                if table.tag != "table":
                    continue
                table_name = table.find("name").text.split('.')[-1]
                columns = [column.find("name").text for column in table if column.tag == "column"]
                tables[table_name] = columns
            gaia_schemas[schema_name] = tables

    return gaia_schemas

def write_data_file(schema, data_path):
    os.makedirs(data_path, exist_ok=True)
    with open(os.path.join(data_path, 'mod.rs'), 'a') as data_file:
        for schema_name in schema:
            data_file.write(f'#[cfg(any({schema_name}, test))]\n')
            data_file.write(f'pub mod {schema_name};\n')

def write_schema_file(data_path, schema_name):
    schema_folder_path = os.path.join(data_path, schema_name)
    os.makedirs(schema_folder_path, exist_ok=True)
    with open(os.path.join(schema_folder_path, 'mod.rs'), 'w') as schema_file:
        schema_file.write(SCHEMA_TEMPLATE.format(name=schema_name, modules="TODO: table_mods", known_tables="TODO: collect known"))

def generate_code(schema):
    data_path = "src/data"
    if os.path.exists(data_path):
        shutil.rmtree(data_path)

    write_data_file(schema, data_path)
            
    for schema_name in schema:
        write_schema_file(data_path, schema_name)

if __name__ == "__main__":
    try:
        schemas = read_xml_file()
        generate_code(schemas)
    except GaiaError as e:
        print(f"An error occurred: {e}")
