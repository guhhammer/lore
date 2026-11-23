# gh ruleset

Gestiona los conjuntos de reglas de los repositorios de GitHub.

## Usage
- `gh rs ls` : muestra todos los conjuntos de reglas del repositorio actual
- `gh rs ls -o nombre_organización` : muestra todos los conjuntos de reglas de una organización específica
- `gh rs check` : comprueba las reglas que se aplican a la rama actual
- `gh rs check nombre_de_la_rama -R propietario/repositorio` : comprueba las reglas que se aplican a una rama específica en otro repositorio
- `gh rs view` : selecciona y muestra de forma interactiva un conjunto de reglas para el repositorio actual
- `gh rs view ruleset_id` : muestra un conjunto de reglas específico por su ID
- `gh rs view ruleset_id -o nombre_organización` : muestra un conjunto de reglas a nivel de organización por su ID
- `gh rs ls -R propietario/repositorio -w` : abre la lista de conjuntos de reglas de un repositorio específico en el navegador

## Links
- More information: https://cli.github.com/manual/gh_ruleset
