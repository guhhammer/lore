# openssl passwd

Генерировать хеш пароля.

## Usage
- `openssl passwd -5` : сгенерировать хеш пароля по алгоритму SHA256 (только Linux)
- `openssl passwd -apr1` : сгенерировать хеш пароля по алгоритму APR1
- `openssl passwd -apr1 -salt строка_соли` : сгенерировать хеш пароля по алгоритму APR1 с солью
- `openssl passwd -apr1 -table` : сгенерировать хеш пароля по алгоритму APR1 и отобразить пароль вместе с хешем
- `echo -n "пароль" | openssl passwd -apr1 -stdin` : сгенерировать хеш пароля из `stdin`

## Links
- More information: https://docs.openssl.org/master/man1/openssl-passwd/
