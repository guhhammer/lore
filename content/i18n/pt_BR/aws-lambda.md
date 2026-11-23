# aws lambda

Usa o AWS Lambda, um serviço de computação para executar código sem provisionar nem gerenciar servidores.

## Usage
- `aws lambda invoke --function-name nome caminho/para/a/resposta.json` : executa uma função
- `aws lambda invoke --function-name nome --payload json caminho/para/a/resposta.json` : executa uma função enviando um payload em formato JSON
- `aws lambda list-functions` : lista as funções
- `aws lambda get-function-configuration --function-name nome` : exibe a configuração de uma função
- `aws lambda list-aliases --function-name nome` : lista os apelidos de uma função
- `aws lambda get-function-concurrency --function-name nome` : exibe a configuração de concorrência reservada de uma função
- `aws lambda get-policy --function-name nome` : lista quais serviços AWS pode invocar a função

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/lambda/
