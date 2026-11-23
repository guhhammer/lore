# aws cur

Crée, requête et supprime des rapports de définition d'utilisation AWS.

## Usage
- `aws cur put-report-definition --report-definition file://chemin/vers/rapport_de_définition.json` : créé un rapport de définition de coût et d'utilisation AWS depuis un fichier JSON
- `aws cur describe-report-definitions` : liste les rapports de définition définis pour le compte connecté
- `aws cur --region region_aws delete-report-definition --report-name rapport` : supprime un rapport de définition d'utilisation

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cur/
