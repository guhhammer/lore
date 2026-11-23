# aws ecr

Push, pull, and manage container images.

## Usage
- `aws ecr get-login-password --region region | docker login --username AWS --password-stdin aws_account_id.dkr.ecr.region.amazonaws.com` : authenticate Docker with the default registry (username is AWS)
- `aws ecr create-repository --repository-name repository --image-scanning-configuration scanOnPush=true|false --region region` : create a repository
- `docker tag container_name:tag aws_account_id.dkr.ecr.region.amazonaws.com/container_name:tag` : tag a local image for ECR
- `docker push aws_account_id.dkr.ecr.region.amazonaws.com/container_name:tag` : push an image to a repository
- `docker pull aws_account_id.dkr.ecr.region.amazonaws.com/container_name:tag` : pull an image from a repository
- `aws ecr batch-delete-image --repository-name repository --image-ids imageTag=latest` : delete an image from a repository
- `aws ecr delete-repository --repository-name repository --force` : delete a repository
- `aws ecr list-images --repository-name repository` : list images within a repository

## Categories
containers, packages, cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ecr/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-ecr
