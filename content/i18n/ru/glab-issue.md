# glab issue

Управлять задачами GitLab.

## Usage
- `glab issue view номер_задачи` : показать конкретную задачу
- `glab issue view номер_задачи -w` : открыть конкретную задачу в браузере по умолчанию
- `glab issue create --web` : создать новую задачу в браузере по умолчанию
- `glab issue list -P 10 -l "bug"` : вывести список последних 10 задач с меткой `bug`
- `glab issue list -c --author имя_пользователя` : вывести список закрытых задач, созданных конкретным пользователем
- `glab issue reopen номер_задачи` : переоткрыть конкретную задачу

## Links
- More information: https://gitlab.com/gitlab-org/cli/-/blob/main/docs/source/issue/_index.md
