# npm shrinkwrap

Фиксировать зависимости пакета, создавая файл `npm-shrinkwrap.json`. Аналогичен `package-lock.json`, но используется для опубликованных пакетов.

## Usage
- `npm shrinkwrap` : сгенерировать файл `npm-shrinkwrap.json` из текущего `package-lock.json`
- `npm shrinkwrap --production` : сгенерировать файл, исключая devDependencies (режим продакшн)
- `npm shrinkwrap --force` : принудительно создать файл, даже если он уже существует

## Links
- More information: https://docs.npmjs.com/cli/v11/npm-shrinkwrap
