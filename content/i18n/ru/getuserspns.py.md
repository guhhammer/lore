# getuserspns.py

Получать имена субъектов-служб (SPN), связанные с учётными записями пользователей Active Directory. Часть набора инструментов Impacket.

## Usage
- `GetUserSPNs.py домен/имя_пользователя:пароль -dc-ip ip_контроллера_домена` : вывести список учётных записей пользователей с SPN и запросить их Kerberos TGS-тикеты
- `GetUserSPNs.py домен/имя_пользователя -hashes LM_хеш:NT_хеш -dc-ip ip_контроллера_домена` : использовать аутентификацию pass-the-hash
- `GetUserSPNs.py домен/имя_пользователя:пароль -dc-ip ip_контроллера_домена -outputfile путь/к/выходному_файлу` : сохранить результаты в файл
- `GetUserSPNs.py домен/имя_пользователя:пароль -dc-ip ip_контроллера_домена -request` : запросить только TGS-тикеты
- `GetUserSPNs.py домен/имя_пользователя -dc-ip ip_контроллера_домена -hashes LM_хеш:NT_хеш -request` : запросить только TGS-тикеты с использованием аутентификации pass-the-hash

## Links
- More information: https://github.com/fortra/impacket
