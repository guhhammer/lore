# az pipelines

Manage Azure Pipelines resources. Part of `azure-cli` (also known as `az`).

## Usage
- `az pipelines create --org organization_url -p project_name --name pipeline_name --description description --repository repository_name --branch branch_name` : create a new Azure Pipeline (YAML based)
- `az pipelines delete --org organization_url -p project_name --id pipeline_id` : delete a specific pipeline
- `az pipelines list --org organization_url -p project_name` : list pipelines
- `az pipelines run --org organization_url -p project_name --name pipeline_name` : enqueue a specific pipeline to run
- `az pipelines show --org organization_url -p project_name --name pipeline_name` : get the details of a specific pipeline
- `az pipelines update --org organization_url -p project_name --name pipeline_name --new-name pipeline_new_name --new-folder-path user1/production_pipelines` : update a specific pipeline
- `az pipelines agent list --org organization_url --pool-id agent_pool` : list all agents in a pool

## Categories
cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/pipelines
- tldr page: https://tldr.inbrowser.app/pages/common/az-pipelines
