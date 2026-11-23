# git switch

Alterna entre ramas Git. Requiere una versión 2.23+ de Git.

## Usage
- `git switch nombre_de_la_rama` : cambia a una rama existente
- `git switch -c nombre_de_la_rama` : crea una nueva rama y se cambia a esta
- `git switch -c nombre_de_la_rama confirmación` : crea y cambia a una nueva rama basada en una confirmación específica
- `git switch -` : cambia a la rama anterior
- `git switch --recurse-submodules nombre_de_la_rama` : cambia a una rama y actualiza todos los submódulos para coincidir
- `git switch -m nombre_de_la_rama` : cambia a una rama y automáticamente fusiona la rama actual y cualquier cambio sin confirmación en ella

## Links
- More information: https://git-scm.com/docs/git-switch
