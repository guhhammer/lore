# gh workflow

List, view, and run GitHub Actions workflows.

## Usage
- `gh workflow view` : interactively select a workflow to view the latest jobs for
- `gh workflow view id|workflow_name|filename.yml -w` : view a specific workflow in the default browser
- `gh workflow view id|workflow_name|filename.yml -y` : display the YAML definition of a specific workflow
- `gh workflow view id|workflow_name|filename.yml -r branch|tag_name -y` : display the YAML definition for a specific Git branch or tag
- `gh workflow ls` : list workflow files (use `--all` to include disabled workflows)
- `gh workflow run id|workflow_name|filename.yml --raw-field param1=value1 --raw-field param2=value2 ...` : run a manual workflow with parameters
- `echo '{"param1": "value1", "param2": "value2", ...}' | gh workflow run id|workflow_name|filename.yml -r branch|tag_name` : run a manual workflow using a specific branch or tag with JSON parameters from `stdin`
- `gh workflow enable|disable id|workflow_name|filename.yml` : enable or disable a specific workflow

## Categories
version-control, text

## Links
- More information: https://cli.github.com/manual/gh_workflow
- tldr page: https://tldr.inbrowser.app/pages/common/gh-workflow
