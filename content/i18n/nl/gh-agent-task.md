# gh agent-task

Beheer GitHub-agenttaken.

## Usage
- `gh agent list` : toon de nieuwste agenttaken
- `gh agent create "Verbeter de prestatie van de dataverwerkingspijplijn"` : maak een nieuwe agenttaak aan voor de huidige repository
- `gh agent create -F pad/naar/bestand` : maak een nieuwe agenttaak aan vanuit een bestand
- `gh agent view sessie_id|pr_nummer|url|branch` : toon details over een specifieke agenttaak
- `gh agent view --log sessie_id|pr_nummer|url|branch` : toon de log van een specifieke agenttaak

## Links
- More information: https://cli.github.com/manual/gh_agent-task
