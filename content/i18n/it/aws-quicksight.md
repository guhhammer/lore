# aws quicksight

Crea, elimina, elenca, cerca e aggiorna entità AWS QuickSight.

## Usage
- `aws quicksight list-data-sets --aws-account-id id_account_aws` : elenca i dataset
- `aws quicksight list-users --aws-account-id id_account_aws --namespace default` : elenca gli utenti
- `aws quicksight list-groups --aws-account-id id_account_aws --namespace default` : elenca i gruppi
- `aws quicksight list-dashboards --aws-account-id id_account_aws` : elenca le dashboard
- `aws quicksight describe-data-set --aws-account-id id_account_aws --data-set-id id_data_set` : visualizza informazioni dettagliate su un dataset
- `aws quicksight describe-data-set-permissions --aws-account-id id_account_aws --data-set-id id_data_set` : visualizza chi ha accesso al dataset e che tipo di azioni può eseguire sul dataset

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/quicksight/
