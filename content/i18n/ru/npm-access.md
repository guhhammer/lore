# npm access

Устанавливать уровень доступа для опубликованных пакетов.

## Usage
- `npm access list packages пользователь|область|область:команда пакет` : вывести список пакетов для пользователя или области видимости
- `npm access list collaborators пакет имя_пользователя` : вывести список соавторов пакета
- `npm access get status пакет` : получить статус пакета
- `npm access set status=public|private пакет` : установить статус пакета (публичный или приватный)
- `npm access grant read-only|read-write область:команда пакет` : выдать доступ к пакету
- `npm access revoke область:команда пакет` : отозвать доступ к пакету
- `npm access set mfa=none|publish|automation пакет` : настроить требование двухфакторной аутентификации

## Links
- More information: https://docs.npmjs.com/cli/npm-access/
