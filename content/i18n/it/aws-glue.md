# aws glue

CLI per AWS Glue. Definisce l'endpoint pubblico per il servizio AWS Glue.

## Usage
- `aws glue list-jobs` : elenca i job
- `aws glue start-job-run --job-name nome_job` : avvia un job
- `aws glue start-workflow-run --name nome_workflow` : avvia l'esecuzione di un workflow
- `aws glue list-triggers` : elenca i trigger
- `aws glue start-trigger --name nome_trigger` : avvia un trigger
- `aws glue create-dev-endpoint --endpoint-name nome --role-arn role_arn_used_by_endpoint` : crea un endpoint di sviluppo

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/glue/
