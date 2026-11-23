# pueue kill

Kill running tasks or whole groups.

## Usage
- `pueue kill` : kill all tasks in the default group
- `pueue kill task_id` : kill a specific task
- `pueue kill --children task_id` : kill a task and terminate all its child processes
- `pueue kill -g group_name` : kill all tasks in a group and pause the group
- `pueue kill -a` : kill all tasks across all groups and pause all groups

## Categories
users

## Links
- More information: https://github.com/Nukesor/pueue#how-to-use-it
- tldr page: https://tldr.inbrowser.app/pages/common/pueue-kill
