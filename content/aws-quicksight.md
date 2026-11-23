# aws quicksight

Create, delete, list, search, and update AWS QuickSight entities.

## Usage
- `aws quicksight list-data-sets --aws-account-id aws_account_id` : list datasets
- `aws quicksight list-users --aws-account-id aws_account_id --namespace default` : list users
- `aws quicksight list-groups --aws-account-id aws_account_id --namespace default` : list groups
- `aws quicksight list-dashboards --aws-account-id aws_account_id` : list dashboards
- `aws quicksight describe-data-set --aws-account-id aws_account_id --data-set-id data_set_id` : display detailed information about a dataset
- `aws quicksight describe-data-set-permissions --aws-account-id aws_account_id --data-set-id data_set_id` : display who has access to the dataset and what kind of actions they can perform on the dataset

## Categories
cloud, users

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/quicksight/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-quicksight
