# topydo

A to-do list application that uses the todo.txt format.

## Usage
- `topydo add "todo_message +project_name @context_name"` : add a to-do to a specific project with a given context
- `topydo add "(A) todo_message due:1d"` : add a to-do with a due date of tomorrow with a priority of `A`
- `topydo add "todo_message due:fri"` : add a to-do with a due date of Friday
- `topydo add "water flowers due:mon rec:1w"` : add a non-strict repeating to-do (next due = now + rec)
- `topydo add "todo_message due:2020-01-01 rec:+1m"` : add a strict repeating to-do (next due = current due + rec)
- `topydo revert` : revert the last `topydo` command executed

## Links
- More information: https://github.com/topydo/topydo
- tldr page: https://tldr.inbrowser.app/pages/common/topydo
