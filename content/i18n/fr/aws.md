# aws

La CLI officielle pour Amazon Web Services. Certaines commandes comme `aws s3` ont leur propre documentation.

## Usage
- `aws configure wizard` : configure la ligne de commande AWS
- `aws configure sso` : configure la ligne de commande AWS en utilisant le SSO
- `aws sts get-caller-identity` : récupère l'identité de l'appelant (utilisé pour débogguer les permissions)
- `aws dynamodb list-tables --region us-east-1 --output yaml` : liste les ressources AWS d'une région et affiche le résultat en YAML
- `aws iam create-user --cli-auto-prompt` : utilise l'aide automatique au remplissage d'une commande
- `aws dynamodb wizard nouvelle_table` : utilise un guide interactif pour une ressource AWS
- `aws dynamodb update-table --generate-cli-skeleton` : génère un squelette CLI en JSON (utile pour l'Infrastructure as Code)
- `aws commande help` : affiche l'aide pour une commande AWS

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/
