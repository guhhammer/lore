# aws sts

Serviço de Token de Segurança (STS) que permite solicitar credenciais temporárias para usuários (IAM) ou federados.

## Usage
- `aws sts assume-role --role-arn arn_do_papel_aws` : obtém credenciais temporárias para acessar recursos AWS específicos
- `aws sts get-caller-identity` : obtém um usuário IAM ou papel que foi usado para chamar a operação

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sts/
