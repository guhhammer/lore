# aws s3 rb

Elimina un bucket S3 vuoto.

## Usage
- `aws s3 rb s3://nome_bucket` : elimina un bucket S3 vuoto
- `aws s3 rb s3://nome_bucket --force` : forza l'eliminazione di un bucket S3 e dei suoi oggetti non versionati (fallirà se sono presenti oggetti versionati)

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/rb.html
