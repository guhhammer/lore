# openssl genrsa

Генерировать приватные ключи RSA.

## Usage
- `openssl genrsa` : сгенерировать приватный ключ RSA длиной 2048 бит в `stdout`
- `openssl genrsa -out выходной_файл.key 1234` : сохранить приватный ключ RSA произвольной длины в выходной файл
- `openssl genrsa -aes256` : сгенерировать приватный ключ RSA и зашифровать его с помощью AES256 (будет запрошена парольная фраза)

## Links
- More information: https://docs.openssl.org/master/man1/openssl-genrsa/
