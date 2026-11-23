# jira sprint

Gestiona sprints en un tablero de proyecto de Jira.

## Usage
- `jira sprint ls` : muestra una lista de sprints y sus incidencias en una vista de explorador
- `jira sprint ls --current` : muestra los problemas del sprint actual
- `jira sprint ls --current -a $(jira me)` : muestra los problemas del sprint actual que me han sido asignados
- `jira sprint ls --current -a $(jira me) -y High` : muestra los problemas de alta prioridad del sprint actual que me han sido asignados
- `jira sprint add` : añade problemas a un sprint mediante un indicador interactivo

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#sprint
