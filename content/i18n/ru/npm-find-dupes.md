# npm find-dupes

Определять дублирующиеся зависимости в `node_modules`.

## Usage
- `npm find-dupes` : вывести список всех дублирующихся пакетов в `node_modules`
- `npm find-dupes --include dev` : включить `devDependencies` в обнаружение дубликатов
- `npm find-dupes имя_пакета` : вывести список всех дублирующихся экземпляров конкретного пакета в `node_modules`
- `npm find-dupes --omit optional` : исключить опциональные зависимости из обнаружения дубликатов
- `npm find-dupes --loglevel silent|error|warn|info|verbose` : установить уровень логирования для вывода
- `npm find-dupes --json` : вывести информацию о дубликатах в формате JSON
- `npm find-dupes --scope @scope1,@scope2` : ограничить поиск дубликатов определёнными областями
- `npm find-dupes --omit-scope @scope1,@scope2` : исключить определённые области из обнаружения дубликатов

## Links
- More information: https://docs.npmjs.com/cli/npm-find-dupes/
