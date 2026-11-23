# aws lambda

Utilizza AWS Lambda, un servizio di calcolo per eseguire codice senza provisionare o gestire server.

## Usage
- `aws lambda invoke --function-name nome percorso/del/response.json` : esegui una funzione
- `aws lambda invoke --function-name nome --payload json percorso/del/response.json` : esegui una funzione con un payload di input in formato JSON
- `aws lambda list-functions` : elenca le funzioni
- `aws lambda get-function-configuration --function-name nome` : visualizza la configurazione di una funzione
- `aws lambda list-aliases --function-name nome` : elenca gli alias delle funzioni
- `aws lambda get-function-concurrency --function-name nome` : visualizza la configurazione di concorrenza riservata per una funzione
- `aws lambda get-policy --function-name nome` : elenca quali servizi AWS possono invocare la funzione

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/lambda/
