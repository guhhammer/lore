# npm cache

Управлять кэшем пакетов npm.

## Usage
- `npm cache add пакет` : добавить конкретный пакет в кэш
- `npm cache clean ключ` : очистить конкретный элемент кэша по ключу
- `npm cache clean -f` : очистить весь кэш npm
- `npm cache ls` : вывести список кэшированных пакетов
- `npm cache ls имя@версия` : вывести список кэшированных пакетов, соответствующих указанному имени и версии
- `npm cache verify` : проверить целостность кэша npm
- `npm cache npx ls` : вывести список всех записей в кэше npx

## Links
- More information: https://docs.npmjs.com/cli/npm-cache/
