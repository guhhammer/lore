# aws dynamodb

Manipulate an AWS DynamoDB database, a fast NoSQL database with predictable performance and seamless scalability.

## Usage
- `aws dynamodb create-table --table-name table_name --attribute-definitions AttributeName=S,AttributeType=S --key-schema AttributeName=S,KeyType=HASH --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5` : create a table
- `aws dynamodb list-tables` : list all tables in the DynamoDB
- `aws dynamodb describe-table --table-name table_name` : get details about a specific table
- `aws dynamodb put-item --table-name table_name --item '{"AttributeName": {"S": "value"\}\}'` : add an item to a table
- `aws dynamodb get-item --table-name table_name --key '{"ID": {"N": "1"\}\}'` : retrieve an item from a table
- `aws dynamodb update-item --table-name table_name --key '{"ID": {"N": "1"\}\}' --update-expression "SET Name = :n" --expression-attribute-values '{":n": {"S": "Jane"\}\}'` : update an item in the table
- `aws dynamodb scan --table-name table_name` : scan items in the table
- `aws dynamodb delete-item --table-name table_name --key '{"ID": {"N": "1"\}\}'` : delete an item from the table

## Categories
databases, cloud, monitoring

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/dynamodb/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-dynamodb
