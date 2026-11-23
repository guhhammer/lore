# npm query

Выводить массив объектов зависимостей с помощью CSS-подобных селекторов.

## Usage
- `npm query ':root > *'` : вывести прямые зависимости
- `npm query ':root > .prod|dev'` : вывести все прямые зависимости продакшн/разработки
- `npm query '#пакет'` : вывести зависимости с определённым именем
- `npm query '#пакет@семантическая_версия'` : вывести зависимости с определённым именем в диапазоне семантической версии
- `npm query ':empty'` : вывести зависимости без собственных зависимостей
- `npm query ":attr(scripts, [postinstall])" | jq 'map(.name) | join("\n")' -r | xargs -I _ npm uninstall _` : найти все зависимости со скриптами postinstall и удалить их
- `npm query ":type(git)" | jq 'map(.name)' | xargs -I _ npm why _` : найти все Git-зависимости и вывести, какое приложение их требует

## Links
- More information: https://docs.npmjs.com/cli/npm-query/
