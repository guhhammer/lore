# aws

Il tool da linea di comando ufficiale per Amazon Web Services. Alcuni sottocomandi, come `s3`, hanno la propria documentazione.

## Usage
- `aws configure wizard` : configura l'AWS Command Line tramite procedura guidata
- `aws configure sso` : configura l'AWS Command Line usando SSO
- `aws sts get-caller-identity` : mostra l'identità del chiamante (utile per diagnosticare problemi di permessi)
- `aws dynamodb list-tables --region us-east-1 --output yaml` : elenca le tabelle DynamoDB in una regione e mostra l'output in YAML
- `aws iam create-user --cli-auto-prompt` : usa il prompt automatico per aiutare nella compilazione di un comando
- `aws dynamodb wizard new_table` : avvia una procedura guidata interattiva per una risorsa AWS
- `aws dynamodb update-table --generate-cli-skeleton` : genera uno scheletro JSON per il CLI (utile per infrastruttura come codice)
- `aws comando help` : mostra la pagina di aiuto per uno specifico comando

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/
