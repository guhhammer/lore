# aws amplify

Development platform for building secure, scalable mobile and web applications.

## Usage
- `aws amplify create-app --name app_name --description description --repository repo_url --platform platform --environment-variables env_vars --tags tags` : create a new Amplify app
- `aws amplify delete-app --app-id app_id` : delete an existing Amplify app
- `aws amplify get-app --app-id app_id` : get details of a specific Amplify app
- `aws amplify list-apps` : list all Amplify apps
- `aws amplify update-app --app-id app_id --name new_name --description new_description --repository new_repo_url --environment-variables new_env_vars --tags new_tags` : update settings of an Amplify app
- `aws amplify create-backend-environment --app-id app_id --environment-name env_name --deployment-artifacts artifacts` : add a new backend environment to an Amplify app
- `aws amplify delete-backend-environment --app-id app_id --environment-name env_name` : remove a backend environment from an Amplify app
- `aws amplify list-backend-environments --app-id app_id` : list all backend environments in an Amplify app

## Categories
cloud, security

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/amplify/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-amplify
