# npm ci

Выполнять чистую установку зависимостей `npm`-проекта для автоматизированных сред. Устанавливать пакеты на основе `package-lock.json` или `npm-shrinkwrap.json`.

## Usage
- `npm ci` : установить зависимости проекта из `package-lock.json` или `npm-shrinkwrap.json`
- `npm ci --omit dev|optional|peer` : установить зависимости проекта, пропуская указанный тип зависимостей
- `npm ci --ignore-scripts` : установить зависимости проекта без выполнения pre-/post-скриптов, определённых в `package.json`

## Links
- More information: https://docs.npmjs.com/cli/npm-ci/
