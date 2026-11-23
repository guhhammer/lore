# aws cur

Crea, interroga ed elimina definizioni di report di utilizzo AWS.

## Usage
- `aws cur put-report-definition --report-definition file://percorso/del/report_definition.json` : crea una definizione di report di costi e utilizzo AWS da un file JSON
- `aws cur describe-report-definitions` : elenca le definizioni di report di utilizzo definite per l'account loggato
- `aws cur --region regione_aws delete-report-definition --report-name report` : elimina una definizione di report di utilizzo

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cur/
