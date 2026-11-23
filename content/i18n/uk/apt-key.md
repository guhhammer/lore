# apt-key

Утиліта керування ключами для диспетчера пакетів APT в Debian та Ubuntu.

## Usage
- `apt-key list` : список довірених ключів
- `apt-key add public_key_file.asc` : додати ключ до довіреного сховища ключів
- `apt-key del key_id` : видалити ключ з довіреного сховища ключів
- `wget -qO - https://host.tld/filename.key | apt-key add -` : додайте віддалений ключ до надійного сховища ключів
- `apt-key adv --keyserver pgp.mit.edu --recv key_id` : додати ключ із сервера ключів лише з ідентифікатором ключа

## Links
- More information: https://manned.org/apt-key
