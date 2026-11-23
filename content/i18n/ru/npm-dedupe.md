# npm dedupe

Уменьшать дублирование в каталоге `node_modules`.

## Usage
- `npm ddp` : дедуплицировать пакеты в `node_modules`
- `npm ddp --lock` : следовать `package-lock.json` или `npm-shrinkwrap.json` при дедупликации
- `npm ddp --strict` : запустить дедупликацию в строгом режиме
- `npm ddp --omit optional|peer` : пропустить опциональные/peer зависимости при дедупликации
- `npm ddp --loglevel verbose` : включить подробное логирование для отладки
- `npm ddp имя_пакета` : ограничить дедупликацию указанным пакетом

## Links
- More information: https://docs.npmjs.com/cli/npm-dedupe/
