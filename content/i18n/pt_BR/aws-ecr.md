# aws ecr

Enviar, buscar, e gerenciar imagens de container.

## Usage
- `aws ecr get-login-password --region region | docker login --username AWS --password-stdin aws_account_id.dkr.ecr.region.amazonaws.com` : autentica o Docker com o registro default (nome do usuário na AWS)
- `aws ecr create-repository --repository-name repository --image-scanning-configuration scanOnPush=true|false --region region` : cria um repositório
- `docker tag nome_do_container:tag aws_account_id.dkr.ecr.region.amazonaws.com/nome_do_container:tag` : cria uma tag em uma imagem local para o ECR
- `docker push aws_account_id.dkr.ecr.region.amazonaws.com/nome_do_container:tag` : envia uma imagem para um repositório
- `docker pull aws_account_id.dkr.ecr.region.amazonaws.com/nome_do_container:tag` : busca a imagem de um repositório
- `aws ecr batch-delete-image --repository-name repositório --image-ids imageTag=latest` : apaga uma imagem de um repositório
- `aws ecr delete-repository --repository-name repositório --force` : apaga um repositório
- `aws ecr list-images --repository-name repositório` : lista as imagens de um repositório

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ecr/
