# aws iam

Interage com o Identity and Access Management (IAM), um serviço web para controlar com segurança o acesso aos serviços da AWS.

## Usage
- `aws iam list-users` : lista os usuários
- `aws iam list-policies` : lista as políticas
- `aws iam list-groups` : lista os grupos
- `aws iam get-group --group-name nome_do_grupo` : obtém os usuários de um grupo
- `aws iam get-policy --policy-arn arn:aws:iam::aws:policy/nome_da_política` : descreve uma política do IAM
- `aws iam list-access-keys` : lista as chaves de acesso
- `aws iam list-access-keys --user-name nome_do_usuário` : lista as chaves de acesso para um usuário específico
- `aws iam help` : mostra ajuda

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/iam/
