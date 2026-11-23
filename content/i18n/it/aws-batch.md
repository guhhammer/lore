# aws batch

Esegue carichi di lavoro di calcolo batch tramite il servizio AWS Batch.

## Usage
- `aws batch list-jobs --job-queue nome_coda` : elenca i lavori batch in esecuzione
- `aws batch create-compute-environment --compute-environment-name nome_compute_environment --type tipo` : crea ambiente di calcolo
- `aws batch create-job-queue --job-queue-name nome_coda --priority priorita' --compute-environment-order compute_environment` : crea coda di lavori batch
- `aws batch submit-job --job-name nome_job --job-queue coda_job --job-definition definizione_job` : invia lavoro
- `aws batch describe-jobs --jobs jobs` : descrivi l'elenco dei lavori batch
- `aws batch cancel-job --job-id id_job --reason motivazione` : annulla lavoro

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/batch/
