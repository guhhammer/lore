# mongoexport

Produce exports of data stored in a MongoDB instance formatted as JSON or CSV.

## Usage
- `mongoexport --uri=connection_string --collection=collection_name` : export a collection to `stdout`, formatted as JSON
- `mongoexport --db=database_name --collection=collection_name --query="query_object" --out=path/to/file.json` : export the documents in the specified collection that match a query to a JSON file
- `mongoexport --collection=collection_name --jsonArray` : export documents as a JSON array instead of one object per line
- `mongoexport --collection=collection_name --type=csv --fields="field1,field2,..." --out=path/to/file.csv` : export documents to a CSV file
- `mongoexport --collection=collection_name --type=csv --fields="field1,field2,..." --queryFile=path/to/file --noHeaderLine --out=path/to/file.csv` : export documents that match the query in the specified file to a CSV file, omitting the list of field names on the first line
- `mongoexport --uri=mongodb_uri --collection=collection_name --pretty` : export documents to `stdout`, formatted as human-readable JSON
- `mongoexport --help` : display help

## Categories
text, files, databases

## Links
- More information: https://www.mongodb.com/docs/database-tools/mongoexport/
- tldr page: https://tldr.inbrowser.app/pages/common/mongoexport
