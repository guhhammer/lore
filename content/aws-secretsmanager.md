# aws secretsmanager

Store, manage, and retrieve secrets.

## Usage
- `aws secretsmanager list-secrets` : show secrets stored by the secrets manager in the current account
- `aws secretsmanager list-secrets --query 'SecretList[*].{Name: Name, ARN: ARN}'` : list all secrets but only show the secret names and ARNs (easy to view)
- `aws secretsmanager create-secret --name name --description "secret_description" --secret-string 'secret'` : create a secret
- `aws secretsmanager delete-secret --secret-id name|arn` : delete a secret (append `--force-delete-without-recovery` to delete immediately without any recovery period)
- `aws secretsmanager describe-secret --secret-id name|arn` : view details of a secret except for secret text
- `aws secretsmanager get-secret-value --secret-id name|arn --version-stage version_of_secret` : retrieve the value of a secret (to get the latest version of the secret omit `--version-stage`)
- `aws secretsmanager rotate-secret --secret-id name|arn --rotation-lambda-arn arn_of_lambda_function` : rotate the secret immediately using a Lambda function
- `aws secretsmanager rotate-secret --secret-id name|arn --rotation-lambda-arn arn_of_lambda_function --rotation-rules AutomaticallyAfterDays=30` : rotate the secret automatically every 30 days using a Lambda function

## Categories
cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/secretsmanager/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-secretsmanager
