# gh agent-task

Gestiona las tareas del agente de GitHub.

## Usage
- `gh agent list` : muestra las tareas del agente más recientes
- `gh agent create "Mejorar el rendimiento del proceso de procesamiento de datos"` : crea una nueva tarea del agente para el repositorio actual
- `gh agent create -F ruta/al/archivo` : crea una nueva tarea de agente a partir de un archivo
- `gh agent view session_id|pr_number|url|branch` : muestra detalles sobre una tarea de agente específica
- `gh agent view --log session_id|pr_number|url|branch` : muestra el registro de una tarea de agente específica

## Links
- More information: https://cli.github.com/manual/gh_agent-task
