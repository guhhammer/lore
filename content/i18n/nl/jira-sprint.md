# jira sprint

Beheer sprints in een Jira-projectbord.

## Usage
- `jira sprint ls` : toon sprints en hun issues in een verkennerweergave
- `jira sprint ls --current` : toon issues van de huidige sprint
- `jira sprint ls --current -a $(jira me)` : toon issues van de huidige sprint, toegewezen aan mij
- `jira sprint ls --current -a $(jira me) -y High` : toon issues met hoge prioriteit van de huidige sprint, toegewezen aan mij
- `jira sprint add` : voeg issues toe aan een sprint met behulp van een interactieve prompt

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#sprint
