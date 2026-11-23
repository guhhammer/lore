# aws kendra

CLI per AWS Kendra.

## Usage
- `aws kendra create-index --name nome --role-arn arn_ruolo` : crea un indice
- `aws kendra list-indexes` : elenca gli indici
- `aws kendra describe-index --id id_indice` : descrivi un indice
- `aws kendra list-data-sources` : elenca le fonti dati
- `aws kendra describe-data-source --id id_sorgente_dati` : descrivi una fonte dati
- `aws kendra get-query-suggestions --index-id id_indice --query-text testo_query` : elenca i suggerimenti di query di ricerca

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/kendra/
