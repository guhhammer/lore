# az repos

Manage Azure DevOps repos. Part of `azure-cli` (also known as `az`).

## Usage
- `az repos list -p project_name` : list all repos in a specific project
- `az repos policy merge-strategy create --repository-id repository_id_in_repos_list --branch branch_name --blocking --enabled --allow-no-fast-forward false --allow-rebase true --allow-rebase-merge true --allow-squash true` : add policy on a specific branch of a specific repository to restrict basic merge
- `az repos policy build create --repository-id repository_id --build-definition-id build_pipeline_id --branch main --blocking --enabled --queue-on-source-update-only true --display-name name --valid-duration minutes` : add build validation on a specific repository, using an existing build pipeline, to be triggered automatically on source update
- `az repos pr list -p project_name -r repository_name --status active` : list all active Pull Requests on a specific repository within a specific project

## Categories
cloud, development, version-control

## Links
- More information: https://learn.microsoft.com/cli/azure/repos
- tldr page: https://tldr.inbrowser.app/pages/common/az-repos
