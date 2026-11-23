# todoist

Access <https://todoist.com> from the command-line.

## Usage
- `todoist a "task_name"` : add a task
- `todoist a "task_name" --priority 1 --label-ids "label_id" --project-name "project_name" --date "tmr 9am"` : add a high priority task with a label, project, and due date
- `todoist q '#project_name "tmr 9am" p1 task_name @label_name'` : add a high priority task with a label, project, and due date in quick mode
- `todoist --header --color list` : list all tasks with a header and color
- `todoist l --filter p1` : list all high priority tasks
- `todoist l --filter '(@label_name | today) & p1'` : list today's tasks with high priority that have the specified label

## Categories
network, processes, development

## Links
- More information: https://github.com/sachaos/todoist#usage
- tldr page: https://tldr.inbrowser.app/pages/common/todoist
