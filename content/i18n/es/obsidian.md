# obsidian

Interfaz de línea de comandos para la aplicación de toma de notas Obsidian Markdown. Nota: Requiere que la aplicación Obsidian esté en ejecución.

## Usage
- `obsidian daily` : abre la nota diaria de hoy
- `obsidian daily:append content="- [ ] tarea"` : añade una tarea a la nota diaria del día actual
- `obsidian create name="nombre_nota" template=nombre_nota` : crea una nueva nota a partir de una plantilla
- `obsidian search query="consulta_búsqueda"` : busca texto en el banco de notas
- `obsidian tasks todo` : muestra todas las tareas pendientes del banco de notas
- `obsidian read file=nombre_nota` : lee una nota concreta
- `obsidian read path=ruta/a/nota.md` : lee una nota en una ruta concreta relativa a la raíz del banco de notas
- `obsidian read` : lee la nota activa

## Links
- More information: https://obsidian.md/help/cli
