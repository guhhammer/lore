# vela

Tools for the Vela pipeline.

## Usage
- `vela add deployment --org organization --repo repository_name --target environment --ref branch|commit|refs/tags/git_tag --description "deploy_description"` : trigger a pipeline to run from a Git branch, commit, or tag
- `vela get deployment --org organization --repo repository_name` : list deployments for a repository
- `vela view deployment --org organization --repo repository_name --deployment deployment_number` : inspect a specific deployment

## Categories
version-control

## Links
- More information: https://go-vela.github.io/docs/reference/cli
- tldr page: https://tldr.inbrowser.app/pages/common/vela
