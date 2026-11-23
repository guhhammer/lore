# tod

A tiny Todoist client in Rust. It takes simple input and dumps it in your inbox or another project. Taking advantage of natural language processing to assign due dates, tags, etc.

## Usage
- `tod project import` : import your projects (this is necessary to enable project prompts)
- `tod --quickadd Buy more milk today` : quickly create a task with due date
- `tod task create` : create a new task (you will be prompted for content and project)
- `tod task create --content "Write more rust" --project code` : create a task in a project
- `tod task next` : get the next task for a project
- `tod task list --scheduled --project work` : get your work schedule
- `tod task list --project work` : get all tasks for work

## Categories
development

## Links
- More information: https://github.com/tod-org/tod/blob/main/docs/usage.md#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/tod
