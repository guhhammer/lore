# pueue

A task management tool for sequential and parallel execution of long-running tasks. Some subcommands such as `add` have their own usage documentation.

## Usage
- `pueue add -- "command"` : add a command to the task list
- `pueue status` : list tasks in the task list
- `pueue send task_id "hello"` : send data to a task's `stdin`
- `pueue log task_id` : view a task's `stdout` and `stderr`, as well as basic information about that task
- `pueue group --add group_name` : create a task group
- `pueue kill task_id` : kill a task
- `pueue parallel number_of_parallel_tasks` : set maximum amount of parallel tasks (queued tasks are started as needed to meet this limit)
- `pueue edit task_id` : edit the command line of a stopped task in the default editor (as specified by `$EDITOR`)

## Categories
monitoring, documents

## Links
- More information: https://github.com/Nukesor/pueue#how-to-use-it
- tldr page: https://tldr.inbrowser.app/pages/common/pueue
