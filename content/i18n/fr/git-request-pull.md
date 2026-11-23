# git request-pull

Générer une requête demandant au projet en amont d'inclure les modifications dans son arborescence.

## Usage
- `git request-pull v1.1 https://example.com/project master` : produire une requête résumant les changements entre la version v1.1 et le master
- `git request-pull v0.1 https://example.com/project master:foo` : produire une requête résumant les changements entre la version v1.1 sur la branche master et la branche locale foo

## Links
- More information: https://git-scm.com/docs/git-request-pull
