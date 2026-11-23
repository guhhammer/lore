# gcrane

Beheertool voor containerafbeeldingen. Deze tool implementeert een superset van de `crane`-commando's, met aanvullende commando's die specifiek zijn voor Google Container Registry (`gcr.io`). Sommige subcommando's zoals `copy`, `gc`, `help`, `ls`, etc. hebben hun eigen documentatie.

## Usage
- `gcrane auth login registry -u gebruiker -p wachtwoord` : log in op een registry
- `gcrane ls registry/project_id` : toon tags, manifesten en sub-repositories
- `gcrane cp -r bronregistry/project_id/repository doelregistry/project_id/repository` : kopieer images van een registry naar een andere
- `gcrane gc registry/project_id/repository` : toon images die door de garbage collector verzameld kunnen worden
- `gcrane gc registry/project_id/repository | xargs -n 1 gcrane delete` : verwijder images die door de garbage collector verzameld kunnen worden
- `gcrane ls gcr.io/mijn-project-id` : toon een specifieke registry met een specifieke ID
- `gcrane cp -r gcr.io/mijn-project-id/repository eu.gcr.io/mijn-project-id/repository` : migreer alle images van de VS-registry naar de EU-registry
