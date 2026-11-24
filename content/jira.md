# jira

Interface (third party) for interacting with Jira. Note: Obtaining a Jira API token and exporting it to the shell as a `$JIRA_API_TOKEN` variable is required.

## Usage
- `jira init` : create a configuration file (required before using `jira`)
- `jira issue ls` : list recent issues
- `jira issue ls -a x -y High` : list unassigned issues with high priority
- `jira sprint ls --current -a $(jira me)` : list issues from the current sprint, assigned to me
- `jira issue create -P parent` : create a new issue with a parent issue
- `jira open 123` : open an issue in the browser

## Categories
network, shell

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#commands
- tldr page: https://tldr.inbrowser.app/pages/common/jira
- Wikipedia: https://en.wikipedia.org/wiki/Jira_(software)
