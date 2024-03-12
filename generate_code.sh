#!/bin/bash

# Create a virtual environment
python -m venv venv

# Activate the virtual environment
source venv/Scripts/activate

# Install necessary modules
pip install requests chardet

# Execute the script
python generate_code.py

# Deactivate the virtual environment
deactivate

cargo fmt
