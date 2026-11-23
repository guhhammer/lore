# gh agent-task

Manage GitHub agent tasks.

## Usage
- `gh agent list` : list the most recent agent tasks
- `gh agent create "Improve the performance of the data processing pipeline"` : create a new agent task for the current repository
- `gh agent create -F path/to/file` : create a new agent task from a file
- `gh agent view session_id|pr_number|url|branch` : view details about a specific agent task
- `gh agent view --log session_id|pr_number|url|branch` : show the log of a specific agent task

## Categories
network, files, monitoring

## Links
- More information: https://cli.github.com/manual/gh_agent-task
- tldr page: https://tldr.inbrowser.app/pages/common/gh-agent-task
