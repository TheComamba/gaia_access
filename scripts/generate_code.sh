#!/bin/bash

# Check if python is available
if command -v python &> /dev/null
then
    PYTHON_CMD=python
elif command -v python3 &> /dev/null
then
    PYTHON_CMD=python3
else
    echo "Python is not installed on your system."
    exit 1
fi

cd $(git rev-parse --show-toplevel)

# Create a virtual environment
$PYTHON_CMD -m venv venv

# Activate the virtual environment
source venv/bin/activate

# Install necessary modules
pip install requests chardet

# Execute the script
$PYTHON_CMD scripts/generate_code.py

# Deactivate the virtual environment
deactivate

cargo fmt
