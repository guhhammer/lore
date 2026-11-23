# zeroclaw cron

Manage scheduled tasks for ZeroClaw.

## Usage
- `zeroclaw cron list` : list all scheduled tasks
- `zeroclaw cron add "* * * * *" "command"` : add a new scheduled task with a cron expression
- `zeroclaw cron once 30m|1h|1d|... "command"` : add a one-shot delayed task
- `zeroclaw cron remove task_id` : remove a scheduled task
- `zeroclaw cron pause task_id` : pause a scheduled task
- `zeroclaw cron resume task_id` : resume a paused task
- `zeroclaw cron -h` : display help

## Categories
system

## Links
- More information: https://github.com/zeroclaw-labs/zeroclaw#quick-start
- tldr page: https://tldr.inbrowser.app/pages/common/zeroclaw-cron
