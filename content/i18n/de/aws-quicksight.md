# aws quicksight

CLI für AWS QuickSight.

## Usage
- `aws quicksight list-data-sets --aws-account-id aws_account_id` : liste alle Datensätze auf
- `aws quicksight list-users --aws-account-id aws_account_id --namespace default` : liste alle Benutzer auf
- `aws quicksight list-groups --aws-account-id aws_account_id --namespace default` : liste alle Gruppen auf
- `aws quicksight list-dashboards --aws-account-id aws_account_id` : liste alle Dashboards auf
- `aws quicksight describe-data-set --aws-account-id aws_account_id --data-set-id datensatz_id` : liste einen Datensatz detailliert aus
- `aws quicksight describe-data-set-permissions --aws-account-id aws_account_id --data-set-id datensatz_id` : liste Zugangsberechtigungen zu einem Datensatz auf

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/quicksight/
