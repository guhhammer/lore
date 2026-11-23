# jira issue

Gestiona incidencias en un proyecto de Jira.

## Usage
- `jira issue ls` : muestra las incidencias recientes
- `jira issue ls -a "correo_electronico_o_mostrar_nombre"` : muestra una lista de incidencias asignadas a un usuario específico
- `jira issue ls -a $(jira me) -y High` : muestra una lista de incidencias de alta prioridad que me han sido asignadas
- `jira issue create` : crea una incidencia mediante un indicador interactivo
- `jira issue edit` : edita un problema mediante un indicador interactivo
- `jira issue asg` : asigna un usuario a un problema mediante un indicador interactivo
- `jira issue mv id_incidencia "En progreso"` : mueve una incidencia a un estado determinado
- `jira issue view id_incidencia` : abre una incidencia en el terminal utilizando `less`

## Links
- More information: https://github.com/ankitpokhrel/jira-cli#issue
