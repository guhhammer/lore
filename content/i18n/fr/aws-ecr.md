# aws ecr

Pousse, récupère et gère les images de conteneur.

## Usage
- `aws ecr get-login-password --region région | docker login --username AWS --password-stdin id_de_compte_aws.dkr.ecr.région.amazonaws.com` : connecte Docker avec le registre par défaut (le nom d'utilisateur est AWS)
- `aws ecr create-repository --repository-name dépôt --image-scanning-configuration scanOnPush=true|false --region région` : crée un dépôt
- `docker tag nom_de_conteneur:tag id_de_compte_aws.dkr.ecr.région.amazonaws.com/nom_de_conteneur:tag` : tag une image locale pour ECR
- `docker push id_de_compte_aws.dkr.ecr.région.amazonaws.com/nom_de_conteneur:tag` : pousse une image dans le dépôt
- `docker pull id_de_compte_aws.dkr.ecr.région.amazonaws.com/nom_de_conteneur:tag` : récupère une image depuis un dépôt
- `aws ecr batch-delete-image --repository-name dépôt --image-ids imageTag=latest` : supprime une image d'un dépôt
- `aws ecr delete-repository --repository-name dépôt --force` : supprime un dépôt
- `aws ecr list-images --repository-name dépôt` : liste les images dans un dépôt

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ecr/
