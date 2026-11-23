# aws ecr

Push, pull e gestisce immagini container.

## Usage
- `aws ecr get-login-password --region regione | docker login --username AWS --password-stdin id_account_aws.dkr.ecr.regione.amazonaws.com` : autentica Docker con il registro predefinito (username è AWS)
- `aws ecr create-repository --repository-name repository --image-scanning-configuration scanOnPush=true|false --region regione` : crea un repository
- `docker tag nome_container:tag id_account_aws.dkr.ecr.regione.amazonaws.com/nome_container:tag` : tagga un'immagine locale per ECR
- `docker push id_account_aws.dkr.ecr.regione.amazonaws.com/nome_container:tag` : push di un'immagine in un repository
- `docker pull id_account_aws.dkr.ecr.regione.amazonaws.com/nome_container:tag` : pull di un'immagine da un repository
- `aws ecr batch-delete-image --repository-name repository --image-ids imageTag=latest` : elimina un'immagine da un repository
- `aws ecr delete-repository --repository-name repository --force` : elimina un repository
- `aws ecr list-images --repository-name repository` : elenca le immagini all'interno di un repository

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ecr/
