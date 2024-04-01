#!/bin/bash

DB_PATH="./db/OUIS.db"
CSV_PATH="./db/ouis.csv"


check_table_exists() {
    sqlite3 "$DB_PATH" "SELECT name FROM sqlite_master WHERE type='table' AND name='manufacturers';"
}

# Create the table and import data if it doesn't exist
setup_db() {
    sqlite3 "$DB_PATH" <<SQL
CREATE TABLE IF NOT EXISTS manufacturers (
    oui TEXT PRIMARY KEY,
    manufacturer TEXT,
    country TEXT
);
SQL

    # Import data from CSV
    sqlite3 "$DB_PATH" <<SQL
.mode csv
.import "$CSV_PATH" manufacturers
.mode column
UPDATE manufacturers SET manufacturer = REPLACE(manufacturer, '"', '');
SQL
}



# Check if the database file exists
if [ ! -f "$DB_PATH" ]; then
    echo "Database does not exist. Creating..."
    setup_db
    echo "Database setup complete."
else
    echo "Database exists."
    # Check if the manufacturers table exists
    if [ -z "$(check_table_exists)" ]; then
        echo "Table 'manufacturers' does not exist. Creating..."
        setup_db
        echo "Table setup complete."
    else
        echo "Table 'manufacturers' already exists. No action needed."
    fi
fi
