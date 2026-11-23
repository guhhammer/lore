# mvn idea

Генерировать файлы проекта IntelliJ IDEA (`.ipr`, `.iml` и `.iws`) для проекта Maven. Примечание: этот плагин устарел. Он больше не поддерживается.

## Usage
- `mvn idea:idea` : сгенерировать все файлы проекта IntelliJ IDEA
- `mvn idea:project` : сгенерировать только файл проекта (`.ipr`)
- `mvn idea:workspace` : сгенерировать только файл рабочей области (`.iws`)
- `mvn idea:module` : сгенерировать только файлы модуля (`.iml`)
- `mvn idea:clean` : удалить все сгенерированные файлы проекта

## Links
- More information: https://maven.apache.org/plugins/maven-idea-plugin/usage.html
