# jira issue

Beheer issues in een Jira-project.

## Usage
- `jira issue ls` : toon recente issues
- `jira issue ls -a "email_of_weergavenaam"` : toon issues toegewezen aan een specifieke gebruiker
- `jira issue ls -a $(jira me) -y High` : toon issues met hoge prioriteit die zijn toegewezen aan mij
- `jira issue create` : maak een issue aan met behulp van een interactieve prompt
- `jira issue edit` : bewerk een issue met behulp van een interactieve prompt
- `jira issue asg` : wijs een gebruiker toe aan een issue met behulp van een interactieve prompt
- `jira issue mv issue_id "In Progress"` : verplaats een issue naar een bepaalde status
- `jira issue view issue_id` : open een issue in de terminal met `less`

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#issue
