# jira sprint

Manage sprints in a Jira project board.

## Usage
- `jira sprint ls` : list sprints and their issues in an explorer view
- `jira sprint ls --current` : list issues from the current sprint
- `jira sprint ls --current -a $(jira me)` : list issues from the current sprint, assigned to me
- `jira sprint ls --current -a $(jira me) -y High` : list high priority issues from the current sprint assigned to me
- `jira sprint add` : add issues to a sprint using an interactive prompt

## Categories
development

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#sprint
- tldr page: https://tldr.inbrowser.app/pages/common/jira-sprint
