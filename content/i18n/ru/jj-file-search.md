# jj file search

Искать содержимое в файлах в репозитории `jj`.

## Usage
- `jj file search -p "regex"` : найти файлы со строками, соответствующими `regex`, в рабочей копии
- `jj file search -p "glob:*шаблон*"` : найти файлы со строками, соответствующими шаблону `glob`, в рабочей копии
- `jj file search -r ревизия -p "шаблон"` : найти содержимое в файлах в конкретной ревизии
- `jj file search -p "шаблон" путь/к/файлу_или_каталогу` : выполнить поиск только в указанных путях или файлах

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-file-search
