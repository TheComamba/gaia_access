import os
import shutil
import chardet
import requests
from xml.etree import ElementTree as ET

FILE_PATH = "dev_data/file.xml"

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

def generate_code(schema):
    folder_path = "src/data"
    
    if os.path.exists(folder_path):
        shutil.rmtree(folder_path)
    
    os.makedirs(folder_path, exist_ok=True)

if __name__ == "__main__":
    try:
        schemas = read_xml_file()
        generate_code(schemas)
    except GaiaError as e:
        print(f"An error occurred: {e}")