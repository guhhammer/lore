# aws-vault

A vault for securely storing and accessing AWS credentials in development environments.

## Usage
- `aws-vault add profile` : add credentials to the secure keystore
- `aws-vault exec profile -- aws s3 ls` : execute a command with AWS credentials in the environment
- `aws-vault login profile` : open a browser window and log in to the AWS Console
- `aws-vault list` : list profiles, along with their credentials and sessions
- `aws-vault rotate profile` : rotate AWS credentials
- `aws-vault remove profile` : remove credentials from the secure keystore

## Categories
cloud

## Links
- More information: https://github.com/99designs/aws-vault
- tldr page: https://tldr.inbrowser.app/pages/common/aws-vault
