# gh config

Change configuration for GitHub CLI.

## Usage
- `gh config get git_protocol` : display what Git protocol is being used
- `gh config set git_protocol ssh` : set protocol to SSH
- `gh config set pager 'delta --side-by-side'` : use `delta` in side-by-side mode as the default pager for all `gh` commands
- `gh config set editor vim` : set text editor to Vim
- `gh config set editor ""` : reset to default text editor
- `gh config set prompt disabled` : disable interactive prompts
- `gh config set key value` : set a specific configuration value

## Categories
editors, network

## Links
- More information: https://cli.github.com/manual/gh_config
- tldr page: https://tldr.inbrowser.app/pages/common/gh-config
