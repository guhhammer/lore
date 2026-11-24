# jira issue

Manage issues in a Jira project.

## Usage
- `jira issue ls` : list recent issues
- `jira issue ls -a "email_or_display_name"` : list issues assigned to a specific user
- `jira issue ls -a $(jira me) -y High` : list high priority issues assigned to me
- `jira issue create` : create an issue using an interactive prompt
- `jira issue edit` : edit an issue using an interactive prompt
- `jira issue asg` : assign user to an issue using an interactive prompt
- `jira issue mv issue_id "In Progress"` : move an issue to a certain state
- `jira issue view issue_id` : open an issue in the terminal using `less`

## Categories
shell, development

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#issue
- tldr page: https://tldr.inbrowser.app/pages/common/jira-issue
- Wikipedia: https://en.wikipedia.org/wiki/Jira_(software)
