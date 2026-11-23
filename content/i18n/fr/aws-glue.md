# aws glue

CLI pour AWS Glue. Définie un endpoint publique pour le service AWS Glue.

## Usage
- `aws glue list-jobs` : liste les tâches
- `aws glue start-job-run --job-name nom_de_la_tâche` : démarre une tâche
- `aws glue start-workflow-run --name nom_flux_de_travail` : démarre un flux de travail
- `aws glue list-triggers` : liste les déclencheurs
- `aws glue start-trigger --name nom_du_déclencheur` : démarre un déclencheur
- `aws glue create-dev-endpoint --endpoint-name nom --role-arn rôle_arn_utilisé_par_l_endpoint` : créé un endpoint de développement

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/glue/
