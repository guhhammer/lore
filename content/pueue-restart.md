# pueue restart

Restart tasks.

## Usage
- `pueue restart task_id` : restart a specific task
- `pueue restart -k task_id task_id` : restart multiple tasks at once, and start them immediately (do not enqueue)
- `pueue restart --edit-path task_id` : restart a specific task from a different path
- `pueue restart -e task_id` : edit a command before restarting
- `pueue restart -i task_id` : restart a task in-place (without enqueuing as a separate task)
- `pueue restart -a --stashed` : restart all failed tasks and stash them

## Links
- More information: https://github.com/Nukesor/pueue#how-to-use-it
- tldr page: https://tldr.inbrowser.app/pages/common/pueue-restart
