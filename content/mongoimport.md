# mongoimport

Import content from a JSON, CSV, or TSV file into a MongoDB database.

## Usage
- `mongoimport --file path/to/file.json --uri mongodb_uri -c collection_name` : import a JSON file into a specific collection
- `mongoimport --type csv --file path/to/file.csv -d database_name -c collection_name` : import a CSV file, using the first line of the file to determine field names
- `mongoimport --jsonArray --file path/to/file.json` : import a JSON array, using each element as a separate document
- `mongoimport --file path/to/file.json --mode delete|merge|upsert --upsertFields "field1,field2,..."` : import a JSON file using a specific mode and a query to match existing documents
- `mongoimport --type csv --file path/to/file.csv --fieldFile path/to/field_file.csv --ignoreBlanks` : import a CSV file, reading field names from a separate CSV file and ignoring fields with empty values
- `mongoimport --help` : display help

## Categories
files, text, databases

## Links
- More information: https://www.mongodb.com/docs/database-tools/mongoimport/
- tldr page: https://tldr.inbrowser.app/pages/common/mongoimport
