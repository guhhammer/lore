# todo

A simple, standards-based, cli todo manager.

## Usage
- `todo list --startable` : list startable tasks
- `todo new thing_to_do -l work` : add a new task to the work list
- `todo edit --location location_name task_id` : add a location to a task with a given ID
- `todo show task_id` : show details about a task
- `todo done task_id1 task_id2 ...` : mark tasks with the specified IDs as completed
- `todo delete task_id` : delete a task
- `todo flush` : delete done tasks and reset the IDs of the remaining tasks

## Links
- More information: https://todoman.readthedocs.io/en/stable/man.html
- tldr page: https://tldr.inbrowser.app/pages/common/todo
