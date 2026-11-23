# az devops

Manage Azure DevOps organizations. Part of `azure-cli` (also known as `az`).

## Usage
- `az devops login --org organization_url` : set the Personal Access Token (PAT) to log in to a particular organization
- `az devops project show -p project_name --open` : open a project in the browser
- `az devops team list-member -p project_name --team team_name` : list members of a specific team working on a particular project
- `az devops configure -l` : check the Azure DevOps CLI current configuration
- `az devops configure -d project=project_name organization=organization_url` : configure the Azure DevOps CLI behavior by setting a default project and a default organization

## Categories
cloud, development

## Links
- More information: https://learn.microsoft.com/cli/azure/devops
- tldr page: https://tldr.inbrowser.app/pages/common/az-devops
