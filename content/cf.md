# cf

Manage apps and services on Cloud Foundry.

## Usage
- `cf login -a api_url` : log in to the Cloud Foundry API
- `cf push app_name` : push an app using the default settings
- `cf marketplace` : view the services available from your organization
- `cf create-service service plan service_name` : create a service instance
- `cf bind-service app_name service_name` : connect an application to a service
- `cf run-task app_name "script_command" --name task_name` : run a script whose code is included in the app, but runs independently
- `cf ssh app_name` : start an interactive SSH session with a VM hosting an app
- `cf logs app_name --recent` : view a dump of recent app logs

## Categories
system, cloud, network

## Links
- More information: https://docs.cloudfoundry.org/cf-cli/getting-started.html
- tldr page: https://tldr.inbrowser.app/pages/common/cf
