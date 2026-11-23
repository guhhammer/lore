# aws s3 website

Imposta la configurazione del sito web per un bucket.

## Usage
- `aws s3 website s3://nome-bucket --index-document index.html` : configura un bucket come sito web statico
- `aws s3 website s3://nome-bucket --index-document index.html --error-document error.html` : configura una pagina di errore per il sito web

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/website.html
