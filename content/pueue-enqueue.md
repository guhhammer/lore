# pueue enqueue

Enqueue stashed tasks.

## Usage
- `pueue enqueue task_id task_id` : enqueue multiple stashed tasks at once
- `pueue enqueue -d 60 task_id` : enqueue a stashed task after 60 seconds
- `pueue enqueue -d wednesday task_id` : enqueue a stashed task next Wednesday
- `pueue enqueue -d "4 months" task_id` : enqueue a stashed task after four months
- `pueue enqueue -d 2021-02-19 task_id` : enqueue a stashed task on 2021-02-19
- `pueue enqueue -h` : list all available date/time formats

## See also
- pueue stash

## Links
- More information: https://github.com/Nukesor/pueue#how-to-use-it
- tldr page: https://tldr.inbrowser.app/pages/common/pueue-enqueue
