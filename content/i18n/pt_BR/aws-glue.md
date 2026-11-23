# aws glue

Linha de comando CLI para o AWS Glue. Define o endpoint público para o servico AWS Glue.

## Usage
- `aws glue list-jobs` : lista trabalhos
- `aws glue start-job-run --job-name nome_do_trabalho` : inicia um trabalho
- `aws glue start-workflow-run --name nome_do_fluxo_de_trabalho` : inicia um fluxo de trabalho
- `aws glue list-triggers` : lista os gatilhos
- `aws glue start-trigger --name nome_do_gatilho` : inicia um gatilho
- `aws glue create-dev-endpoint --endpoint-name nome --role-arn papel_arn_usado_pelo_endpoint` : cria um endpoint de desenvolvimento

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/glue/
