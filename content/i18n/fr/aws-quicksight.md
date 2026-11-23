# aws quicksight

Crée, supprime, liste, recherche, et met à jour les entités AWS QuickSight.

## Usage
- `aws quicksight list-data-sets --aws-account-id id_compte_aws` : liste les datasets
- `aws quicksight list-users --aws-account-id id_compte_aws --namespace default` : liste les utilisateurs
- `aws quicksight list-groups --aws-account-id id_compte_aws --namespace default` : liste les groupes
- `aws quicksight list-dashboards --aws-account-id id_compte_aws` : liste les tableaux de bords
- `aws quicksight describe-data-set --aws-account-id id_compte_aws --data-set-id id_data_set` : affiche les informations détaillées sur un dataset
- `aws quicksight describe-data-set-permissions --aws-account-id id_compte_aws --data-set-id id_data_set` : affiche les personnes qui peuvent accéder au dataset et quelles actions ils peuvent effectuer sur ce dernier

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/quicksight/
