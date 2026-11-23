# git commit-graph

Записує та перевіряє файл графіку комітів Git.

## Usage
- `git commit-graph write` : записує файл графіку комітів для спакованих комітів у локальній директорії `.git`
- `git show-ref -s | git commit-graph write --stdin-commits` : записує файл графіку комітів, що містить набір усіх досяжних комітів
- `git rev-parse HEAD | git commit-graph write --stdin-commits --append` : записує файл графіку комітів, що містить усі коміти у поточному файлі графіку комітів разом з тими, до яких можна отримати доступ з `HEAD`

## Links
- More information: https://git-scm.com/docs/git-commit-graph
