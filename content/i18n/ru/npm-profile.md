# npm profile

Управлять профилем npm и связанными настройками. Примечание: эта команда не поддерживает рабочие области.

## Usage
- `npm profile get` : показать данные профиля npm
- `npm profile get свойство` : получить значение конкретного свойства профиля
- `npm profile set свойство значение` : установить или обновить свойство профиля
- `npm profile set email email` : установить публичный адрес электронной почты
- `npm profile set fullname имя` : установить публичное имя
- `npm profile set password` : установить новый пароль в интерактивном режиме
- `npm profile enable-2fa auth-only|auth-and-writes` : включить двухфакторную аутентификацию (2FA) (по умолчанию `auth-and-writes`)
- `npm profile disable-2fa` : отключить двухфакторную аутентификацию (2FA)

## Links
- More information: https://docs.npmjs.com/cli/npm-profile/
