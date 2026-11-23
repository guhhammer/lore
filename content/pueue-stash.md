# pueue stash

Stash tasks to prevent them starting automatically.

## Usage
- `pueue stash task_id` : stash an enqueued task
- `pueue stash task_id task_id` : stash multiple tasks at once
- `pueue start task_id` : start a stashed task immediately
- `pueue enqueue task_id` : enqueue a task to be executed when preceding tasks finish

## See also
- pueue start
- pueue enqueue

## Links
- More information: https://github.com/Nukesor/pueue#how-to-use-it
- tldr page: https://tldr.inbrowser.app/pages/common/pueue-stash
