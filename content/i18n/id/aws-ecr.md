# aws ecr

Dorong, tarik, dan atur citra kontainer.

## Usage
- `aws ecr get-login-password --region wilayah | docker login --username AWS --password-stdin id_akun_aws.dkr.ecr.wilayah.amazonaws.com` : masuk ke dalam manajer Docker dengan registri default (username adalah AWS)
- `aws ecr create-repository --repository-name repositori --image-scanning-configuration scanOnPush=true|false --region wilayah` : buat suatu repositori
- `docker tag nama_kontainer:tag id_akun_aws.dkr.ecr.wilayah.amazonaws.com/nama_kontainer:tag` : tandai suatu citra lokal untuk ECR
- `docker push id_akun_aws.dkr.ecr.wilayah.amazonaws.com/nama_kontainer:tag` : dorong suatu citra menuju suatu repositori
- `docker pull id_akun_aws.dkr.ecr.wilayah.amazonaws.com/nama_kontainer:tag` : tarik suatu citra dari suatu repositori
- `aws ecr batch-delete-image --repository-name repositori --image-ids imageTag=latest` : hapus suatu citra dalam suatu repositori
- `aws ecr delete-repository --repository-name repositori --force` : hapus suatu repositori
- `aws ecr list-images --repository-name repositori` : tampilkan daftar citra dalam suatu repositori

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ecr/
