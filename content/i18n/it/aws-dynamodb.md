# aws dynamodb

Manipola un database AWS DynamoDB, un database NoSQL veloce con prestazioni prevedibili e scalabilità senza soluzione di continuità.

## Usage
- `aws dynamodb create-table --table-name nome_table --attribute-definitions AttributeName=S,AttributeType=S --key-schema AttributeName=S,KeyType=HASH --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5` : crea una tabella
- `aws dynamodb list-tables` : elenca tutte le tabelle in DynamoDB
- `aws dynamodb describe-table --table-name nome_table` : ottieni dettagli su una tabella specifica
- `aws dynamodb put-item --table-name nome_table --item '{"AttributeName": {"S": "valore"\}\}'` : aggiunge un elemento a una tabella
- `aws dynamodb get-item --table-name nome_table --key '{"ID": {"N": "1"\}\}'` : recupera un elemento da una tabella
- `aws dynamodb update-item --table-name nome_table --key '{"ID": {"N": "1"\}\}' --update-expression "SET Name = :n" --expression-attribute-values '{":n": {"S": "Jane"\}\}'` : aggiorna un elemento nella tabella
- `aws dynamodb scan --table-name nome_table` : scansiona gli elementi nella tabella
- `aws dynamodb delete-item --table-name nome_table --key '{"ID": {"N": "1"\}\}'` : elimina un elemento dalla tabella

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/dynamodb/
