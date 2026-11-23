# npm token

Управлять токенами аутентификации реестра npm и генерировать их.

## Usage
- `npm token create` : создать новый токен аутентификации
- `npm token list` : вывести список всех токенов, связанных с учётной записью
- `npm token revoke идентификатор_токена` : отозвать определённый токен по его идентификатору
- `npm token create --read-only` : создать токен с доступом только для чтения
- `npm token create --publish` : создать токен с доступом на публикацию
- `npm login` : автоматически настроить токен npm в глобальном файле `.npmrc` при входе

## Links
- More information: https://docs.npmjs.com/cli/npm-token/
