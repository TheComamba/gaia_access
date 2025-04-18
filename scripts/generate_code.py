import copy
import os
import shutil
import chardet
import requests
from xml.etree import ElementTree as ET

FILE_PATH = "dev_data/tables.xml"

DATA_TEMPLATE = """
// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known schemas and tables in the Gaia database.

{schema_modules}

#[cfg(test)]
/// Collects all the known schemas and tables in the Gaia database.
pub(crate) fn collect_known_schemas(
) -> std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>> {{
    let mut known = std::collections::HashMap::new();
    {known_schemas}
    known
}}
"""

SCHEMA_TEMPLATE = """
// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known tables in the {name} schema.

use crate::traits::Schema;

{description}
#[allow(non_camel_case_types)]
pub struct {name};

impl Schema for {name} {{
    fn string(&self) -> String {{
        "{name}".to_string()
    }}
}}

{modules}

#[cfg(test)]
/// Collects all the known tables in the {name} schema.
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
// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the {name} table.

use crate::traits::{{Column, Table}};

{description}
#[allow(non_camel_case_types)]
pub struct {name};

impl Table for {name} {{
    fn string(&self) -> String {{
        "{name}".to_string()
    }}
}}

/// The columns in the {name} table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {{
    {columns}
}}

impl Column for Col {{}}

#[cfg(test)]
/// Collects all the known columns in the {name} table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {{
    let mut col_strings = Vec::new();
    {known_columns}
    map.insert({name}.string(), col_strings);
}}
"""

class GaiaError(Exception):
    pass

def check_and_download_file():
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
            if schema.find("description") is None:
                schema_description = "/// The " + schema_name + " schema. (No further description available)"
            else:
                schema_description = "/// " + schema.find("description").text.replace("\n", "\n/// ")
            tables = {}
            for table in schema:
                if table.tag != "table":
                    continue
                table_name = table.find("name").text.split('.')[-1]
                if table.find("description") is None:
                    table_description = "/// The " + table_name + " table. (No further description available)"
                else:
                    table_description = "/// " + table.find("description").text.replace("\n", "\n/// ")
                columns = []
                for column in table:
                    if column.tag != "column":
                        continue
                    column_name = column.find("name").text
                    if column.find("description") is None:
                        column_description = "/// The " + column_name + " column. (No further description available)"
                    else:
                        column_description = "/// " + column.find("description").text.replace("\n", "\n/// ")
                    columns.append({ "name": column_name, "description": column_description })
                tables[table_name] = { "columns": columns, "description": table_description }
            gaia_schemas[schema_name] = { "tables": tables, "description": schema_description }

    return gaia_schemas

def get_all_table_features(schema, schema_name):
    if schema_name not in schema:
        raise ValueError(f"Schema '{schema_name}' does not exist in the provided schema dictionary.")
    all_table_features = []
    for table in schema[schema_name]["tables"].keys():
        all_table_features.append(f"{schema_name}_{table}")
    return all_table_features

def write_data_file(schema, data_path):
    os.makedirs(data_path, exist_ok=True)

    schema_mods = []
    known_schemas = []
    for schema_name in schema.keys():
        tables = schema[schema_name]["tables"]
        if tables:
            table_features = get_all_table_features(schema, schema_name)
            table_features = ', '.join([f'feature = "{feature}"' for feature in table_features])
            table_features += ', '
        else:
            table_features = ''
        schema_mods.append(f'#[cfg(not(doctest))]') # Markdown specification states that sections indented by a tab or four spaces are code
        schema_mods.append(f'#[cfg(any(feature = "{schema_name}", {table_features} test))] ')
        schema_mods.append(f'pub mod {schema_name};')
        known_schemas.append(f"{schema_name}::collect_known(&mut known);")

    schema_mods = "\n".join(schema_mods)
    known_schemas = "\n".join(known_schemas)

    with open(os.path.join(data_path, 'mod.rs'), 'w') as data_file:
        data_file.write(DATA_TEMPLATE.format(schema_modules=schema_mods, known_schemas=known_schemas))

def write_schema_file(schema_folder_path, schema_name, schema_description, tables):
    os.makedirs(schema_folder_path, exist_ok=True)

    table_mods = "\n".join([f'#[cfg(any(feature = "{schema_name}_{table}", test))] pub mod {table};' for table in tables])
    known_tables = "\n".join([f"{table}::collect_known(&mut tables);" for table in tables])

    with open(os.path.join(schema_folder_path, 'mod.rs'), 'w') as schema_file:
        schema_file.write(SCHEMA_TEMPLATE.format(name=schema_name, description=schema_description, modules=table_mods, known_tables=known_tables))

def write_table_file(table_folder_path, table_name, table_description, columns):
    os.makedirs(table_folder_path, exist_ok=True)

    columns_for_enum = copy.deepcopy(columns)
    columns_for_known = copy.deepcopy(columns)

    for column in columns_for_enum:
        if column["name"] == "\"size\"":
            column["name"] = "#[strum(serialize = \"\\\"size\\\"\")] size"
        if column["name"] == "type":
            column["name"] = "#[strum(serialize = \"type\")] type_col"
        if column["name"] == "\"key\"":
            column["name"] = "#[strum(serialize = \"\\\"key\\\"\")] key"
        if column["name"] == "\"value\"":
            column["name"] = "#[strum(serialize = \"\\\"value\\\"\")] value"

    for column in columns_for_known:
        if column["name"] == "\"size\"":
            column["name"] = "size"
        if column["name"] == "type":
            column["name"] = "type_col"
        if column["name"] == "\"key\"":
            column["name"] = "key"
        if column["name"] == "\"value\"":
            column["name"] = "value"

    column_enums = "\n".join([f'{column["description"]}\n{column["name"]},' for column in columns_for_enum])
    known_columns = "\n".join([f'col_strings.push(Col::{column["name"]}.to_string());' for column in columns_for_known])

    with open(os.path.join(table_folder_path, 'mod.rs'), 'w') as table_file:
        table_file.write(TABLE_TEMPLATE.format(name=table_name, description=table_description, columns=column_enums, known_columns=known_columns))

import os

def update_cargo_toml(schema):
    if not os.path.exists('Cargo.toml'):
        print("Error: Cargo.toml file does not exist.")
        return

    try:
        with open('Cargo.toml', 'r') as file:
            lines = file.readlines()
    except Exception as e:
        print(f"Error reading Cargo.toml: {e}")
        return

    try:
        begin_index = lines.index('# begin generated\n') + 1
        end_index = lines.index('# end generated\n')
    except ValueError:
        print("Error: Cannot find the required comments in Cargo.toml.")
        return

    new_lines = []
    for schema_name in schema.keys():
        table_features = get_all_table_features(schema, schema_name)
        new_lines.append(f"{schema_name} = {table_features}\n")
        for table_feature in table_features:
            new_lines.append(f"{table_feature} = []\n")

    lines[begin_index:end_index] = new_lines

    try:
        with open('Cargo.toml', 'w') as file:
            file.writelines(lines)
    except Exception as e:
        print(f"Error writing to Cargo.toml: {e}")

def generate_code(schema):
    data_path = "src/data"
    if os.path.exists(data_path):
        shutil.rmtree(data_path)

    write_data_file(schema, data_path)
            
    for schema_name in schema.keys():
        tables = schema[schema_name]["tables"]
        schema_folder_path = os.path.join(data_path, schema_name)
        schema_description = schema[schema_name]["description"]
        write_schema_file(schema_folder_path, schema_name, schema_description, tables)
        for table_name in tables.keys():
            columns = tables[table_name]["columns"]
            table_folder_path = os.path.join(schema_folder_path, table_name)
            table_description = tables[table_name]["description"]
            write_table_file(table_folder_path, table_name, table_description, columns)

    update_cargo_toml(schema)

if __name__ == "__main__":
    try:
        schemas = read_xml_file()
        generate_code(schemas)
    except GaiaError as e:
        print(f"An error occurred: {e}")
