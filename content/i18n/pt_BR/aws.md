# aws

A interface de linha de comando oficial para o Amazon Web Services. Passo-a-passo, Single Sign-On (SSO), autocompletar de recursos e opções de YAML somente na v2. Alguns subcomandos como `s3` tem sua própia documentação de uso.

## Usage
- `aws configure wizard` : configura a linha de comando da AWS
- `aws configure sso` : configura a linha de comando da AWS usando o SSO
- `aws sts get-caller-identity` : obtenha a informações da identidade usada (útil para analisar problemas de permissão)
- `aws dynamodb list-tables --region sa-east-1 --output yaml` : lista recursos da AWS em uma região em yaml
- `aws iam create-user --cli-auto-prompt` : usa prompt de comando para ajuda com o preenchimento
- `aws dynamodb wizard nova_tabela` : usa um passo-a-passo interativo para um recurso da AWS
- `aws dynamodb update-table --generate-cli-skeleton` : gera um arquivo esqueleo em JSON (útil para ser usado em infraestrutura como código)
- `aws comando help` : veja o texto de ajuda para o comando da AWS

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/
