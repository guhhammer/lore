# aws s3

CLI pour AWS S3 - fournit du stockage à travers des interfaces de services web. Certaines sous-commandes, telles que `cp`, disposent de leur propre documentation d'utilisation.

## Usage
- `aws s3 ls nom_bucket` : affiche les fichiers d'un bucket
- `aws s3 sync chemin/vers/répertoire s3://nom_bucket` : synchronise les fichiers dans un répertoire local vers un bucket
- `aws s3 sync s3://nom_bucket chemin/vers/cible` : synchronise les fichiers et répertoires d'un bucket vers un répertoire local
- `aws s3 sync chemin/vers/répertoire s3://nom_bucket --exclude chemin/vers/fichier --exclude chemin/vers/répertoire/*` : synchronise les fichiers dans un répertoire avec des exclusions
- `aws s3 rm s3://bucket/chemin/vers/fichier` : supprime un fichier d'un bucket
- `aws s3 commande --dryrun` : simule une commande sans apporter de modifications

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/
