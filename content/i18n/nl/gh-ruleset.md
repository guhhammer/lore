# gh ruleset

Beheer GitHub repository-rulesets.

## Usage
- `gh rs ls` : toon alle rulesets voor de huidige repository
- `gh rs ls -o organisatie_naam` : toon alle rulesets voor een specifieke organisatie
- `gh rs check` : controleer de regels die van toepassing zijn op de huidige branch
- `gh rs check branch_naam -R eigenaar/repository` : controleer de regels die van toepassing zijn op een specifieke branch in een andere repository
- `gh rs view` : selecteer en bekijk interactief een ruleset voor de huidige repository
- `gh rs view ruleset_id` : bekijk een specifieke ruleset via zijn ID
- `gh rs view ruleset_id -o organisatie_naam` : bekijk een ruleset op organisatieniveau via zijn ID
- `gh rs ls -R eigenaar/repository -w` : open de lijst met rulesets voor een specifieke repository in de browser

## Links
- More information: https://cli.github.com/manual/gh_ruleset
