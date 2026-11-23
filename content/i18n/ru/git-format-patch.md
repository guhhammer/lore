# git format-patch

Подготавливать файлы `.patch`. Полезно при отправке коммитов по электронной почте.

## Usage
- `git format-patch origin` : создать автоматически именованный файл `.patch` для всех неотправленных коммитов
- `git format-patch ревизия_1..ревизия_2` : записать файл `.patch` для всех коммитов между двумя ревизиями в `stdout`
- `git format-patch -n` : записать файл `.patch` для `n` последних коммитов

## Links
- More information: https://git-scm.com/docs/git-format-patch
