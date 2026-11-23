# apt-key

Утилита управления ключами для диспетчера пакетов APT в Debian и Ubuntu.

## Usage
- `apt-key list` : список доверенных ключей
- `apt-key add файл_публичного_ключа.asc` : добавить ключ в список доверенных ключей
- `apt-key del идентификатор_ключа` : удалить ключ из списка доверенных ключей
- `wget -qO - https://host.tld/файл.key | apt-key add -` : добавить удалённый ключ в список доверенных ключей
- `apt-key adv --keyserver pgp.mit.edu --recv идентификатор_ключа` : добавить ключ с сервера ключей, указав только идентификатор ключа

## Links
- More information: https://manned.org/apt-key
