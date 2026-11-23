# pueue pause

Pause running tasks or groups.

## Usage
- `pueue pause` : pause all tasks in the default group
- `pueue pause task_id` : pause a running task
- `pueue pause --children task_id` : pause a running task and stop all its direct children
- `pueue pause -g group_name` : pause all tasks in a group and prevent it from starting new tasks
- `pueue pause -a` : pause all tasks and prevent all groups from starting new tasks

## See also
- pueue start

## Categories
users

## Links
- More information: https://github.com/Nukesor/pueue#how-to-use-it
- tldr page: https://tldr.inbrowser.app/pages/common/pueue-pause
