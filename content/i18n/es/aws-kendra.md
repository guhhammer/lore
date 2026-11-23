# aws kendra

CLI para AWS Kendra.

## Usage
- `aws kendra create-index --name nombre --role-arn rol_arn` : crea un índice
- `aws kendra list-indexes` : lista índices
- `aws kendra describe-index --id id_índice` : describe un índice
- `aws kendra list-data-sources` : lista fuentes de datos
- `aws kendra describe-data-source --id id_fuente_datos` : describe una fuente de datos
- `aws kendra get-query-suggestions --index-id id_índice --query-text texto_consulta` : lista consultas de búsqueda

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/kendra/
