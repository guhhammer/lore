# npm adduser

Добавлять учётную запись пользователя реестра.

## Usage
- `npm adduser --registry url_реестра` : создать нового пользователя в указанном реестре и сохранить учётные данные в `.npmrc`
- `npm login --scope @организация --registry https://registry.example.com` : войти в приватный реестр с определённой областью видимости
- `npm logout --scope @организация` : выйти из определённой области видимости и удалить токен аутентификации
- `npm init --scope @организация -y` : создать пакет с областью видимости при инициализации

## Links
- More information: https://docs.npmjs.com/cli/v11/npm-adduser
