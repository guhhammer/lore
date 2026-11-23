# aws lambda

CLI pour AWS lambda.

## Usage
- `aws lambda invoke --function-name nom chemin/vers/réponse.json` : lance une fonction
- `aws lambda invoke --function-name nom --payload json chemin/vers/réponse.json` : lance une fonction avec pour donnée d'entrée, un document JSON
- `aws lambda list-functions` : liste les fonctions
- `aws lambda get-function-configuration --function-name nom` : affiche la configuration d'une fonction
- `aws lambda list-aliases --function-name nom` : affiche les alias d'une fonction
- `aws lambda get-function-concurrency --function-name nom` : affiche la configuration de concurrence pour une fonction
- `aws lambda get-policy --function-name nom` : affiche quel service AWS peut appeler une fonction

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/lambda/
