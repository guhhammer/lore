# aws cognito-idp

Configure an Amazon Cognito user pool and its users and groups and authenticate them.

## Usage
- `aws cognito-idp create-user-pool --pool-name name` : create a new Cognito user pool
- `aws cognito-idp list-user-pools --max-results 10` : list all user pools
- `aws cognito-idp delete-user-pool --user-pool-id user_pool_id` : delete a specific user pool
- `aws cognito-idp admin-create-user --username username --user-pool-id user_pool_id` : create a user in a specific pool
- `aws cognito-idp list-users --user-pool-id user_pool_id` : list the users of a specific pool
- `aws cognito-idp admin-delete-user --username username --user-pool-id user_pool_id` : delete a user from a specific pool

## Categories
users, cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cognito-idp/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-cognito-idp
