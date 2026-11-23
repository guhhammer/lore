# npm sbom

Генерировать спецификацию программных компонентов (SBOM) для Node.js-проекта.

## Usage
- `npm sbom` : вывести список всех зависимостей в проекте
- `npm sbom --omit dev --omit optional` : исключить зависимости `dev` и `optional`
- `npm sbom --package-lock-only` : сгенерировать SBOM на основе только `package-lock.json`

## Links
- More information: https://docs.npmjs.com/cli/npm-sbom/
