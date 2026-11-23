# aws sqs

Crea, elimina e invia messaggi alle code per il servizio AWS SQS.

## Usage
- `aws sqs list-queues` : elenca tutte le code disponibili
- `aws sqs get-queue-url --queue-name nome_coda` : visualizza l'URL di una coda specifica
- `aws sqs create-queue --queue-name nome_coda --attributes file://percorso/del/attributes_file.json` : crea una coda con attributi specifici da un file in formato JSON
- `aws sqs send-message --queue-url https://sqs.regione.amazonaws.com/nome_coda --message-body "body_messaggio" --delay-seconds ritardo --message-attributes file://percorso/del/attributes_file.json` : invia un messaggio specifico a una coda
- `aws sqs delete-message --queue-url https://url_coda --receipt-handle receipt_handle` : elimina il messaggio specificato da una coda
- `aws sqs delete-queue --queue-url https://sqs.regione.amazonaws.com/nome_coda` : elimina una coda specifica
- `aws sqs purge-queue --queue-url https://sqs.regione.amazonaws.com/nome_coda` : elimina tutti i messaggi dalla coda specificata
- `aws sqs add-permission --queue-url https://sqs.regione.amazonaws.com/nome_coda --label permission_name --aws-account-ids account_id --actions SendMessage` : abilita un account AWS specifico a inviare messaggi alla coda

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sqs/
