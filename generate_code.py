import os
import shutil
import chardet
import requests
from xml.etree import ElementTree as ET

FILE_PATH = "dev_data/tables.xml"

DATA_TEMPLATE = """
// This code is generated by generate_code.py, do not modify it manually
{schema_modules}

#[cfg(test)]
pub(crate) fn collect_known_schemas(
) -> std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>> {{
    let mut known = std::collections::HashMap::new();
    {known_schemas}
    known
}}
"""

SCHEMA_TEMPLATE = """
// This code is generated by generate_code.py, do not modify it manually
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
    // Some tables do not have any columns. Disabling compiler warnings for these cases
    #[allow(unused_mut)]
    let mut tables = std::collections::HashMap::new();
    {known_tables}
    map.insert({name}.string(), tables);
}}
"""

TABLE_TEMPLATE = """
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct {name};

impl Table for {name} {{
    fn string(&self) -> String {{
        "{name}".to_string()
    }}
}}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {{
    {columns}
}}

impl Column for Col {{}}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {{
    let mut col_strings = Vec::new();
    {known_columns}
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

        with open(FILE_PATH, 'wb') as file:
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

def get_all_table_features(schema, schema_name):
    if schema_name not in schema:
        raise ValueError(f"Schema '{schema_name}' does not exist in the provided schema dictionary.")
    all_table_features = []
    for table in schema[schema_name].keys():
        all_table_features.append(f"{schema_name}_{table}")
    return all_table_features

def write_data_file(schema, data_path):
    os.makedirs(data_path, exist_ok=True)

    schema_mods = []
    known_schemas = []
    for schema_name, tables in schema.items():
        if tables:
            table_features = get_all_table_features(schema, schema_name)
            table_features.append('')
            table_features = ', '.join(table_features)
        else:
            table_features = ''
        schema_mods.append(f"#[cfg(any({schema_name}, {table_features}test))] pub mod {schema_name};")
        known_schemas.append(f"{schema_name}::collect_known(&mut known);")

    schema_mods = "\n".join(schema_mods)
    known_schemas = "\n".join(known_schemas)

    with open(os.path.join(data_path, 'mod.rs'), 'w') as data_file:
        data_file.write(DATA_TEMPLATE.format(schema_modules=schema_mods, known_schemas=known_schemas))

def write_schema_file(schema_folder_path, schema_name, tables):
    os.makedirs(schema_folder_path, exist_ok=True)

    table_mods = "\n".join([f"#[cfg(any({schema_name}_{table}, test))] pub mod {table};" for table in tables])
    known_tables = "\n".join([f"{table}::collect_known(&mut tables);" for table in tables])

    with open(os.path.join(schema_folder_path, 'mod.rs'), 'w') as schema_file:
        schema_file.write(SCHEMA_TEMPLATE.format(name=schema_name, modules=table_mods, known_tables=known_tables))

def write_table_file(table_folder_path, table_name, columns):
    os.makedirs(table_folder_path, exist_ok=True)

    columns_for_enum = columns.copy()
    columns_for_known = columns.copy()

    columns_for_enum = ["#[strum(serialize = \"\\\"size\\\"\")] size" if column == "\"size\"" else column for column in columns_for_enum]
    columns_for_known = ["size" if column == "\"size\"" else column for column in columns_for_known]

    columns_for_enum = ["#[strum(serialize = \"type\")] type_col" if column == "type" else column for column in columns_for_enum]
    columns_for_known = ["type_col" if column == "type" else column for column in columns_for_known]

    columns_for_enum = ["#[strum(serialize = \"\\\"key\\\"\")] key" if column == "\"key\"" else column for column in columns_for_enum]
    columns_for_known = ["key" if column == "\"key\"" else column for column in columns_for_known]

    columns_for_enum = ["#[strum(serialize = \"\\\"value\\\"\")] value" if column == "\"value\"" else column for column in columns_for_enum]
    columns_for_known = ["value" if column == "\"value\"" else column for column in columns_for_known]

    column_enums = "\n".join([f"{column}," for column in columns_for_enum])
    known_columns = "\n".join([f"col_strings.push(Col::{column}.to_string());" for column in columns_for_known])

    with open(os.path.join(table_folder_path, 'mod.rs'), 'w') as table_file:
        table_file.write(TABLE_TEMPLATE.format(name=table_name, columns=column_enums, known_columns=known_columns))

def generate_code(schema):
    data_path = "src/data"
    if os.path.exists(data_path):
        shutil.rmtree(data_path)

    write_data_file(schema, data_path)
            
    for schema_name, tables in schema.items():
        schema_folder_path = os.path.join(data_path, schema_name)
        write_schema_file(schema_folder_path, schema_name, tables)
        for table_name, columns in tables.items():
            table_folder_path = os.path.join(schema_folder_path, table_name)
            write_table_file(table_folder_path, table_name, columns)

if __name__ == "__main__":
    try:
        schemas = read_xml_file()
        generate_code(schemas)
    except GaiaError as e:
        print(f"An error occurred: {e}")
