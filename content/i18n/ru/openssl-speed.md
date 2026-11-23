# openssl speed

Тестировать производительность алгоритмов OpenSSL.

## Usage
- `openssl speed` : протестировать все доступные алгоритмы
- `openssl speed md5 sha1 sha256 aes rsa` : протестировать наиболее распространённые алгоритмы
- `openssl speed -evp aes-256-cbc` : протестировать конкретный шифр EVP
- `openssl speed -seconds 10` : протестировать алгоритмы в течение указанного времени (в секундах)
- `openssl speed -elapsed` : протестировать с использованием реального времени вместо пользовательского процессорного времени
- `openssl speed -mr` : вывести результат в машиночитаемом формате
- `openssl speed -multi $(nproc)` : протестировать несколько алгоритмов параллельно, используя все доступные ядра процессора

## Links
- More information: https://docs.openssl.org/master/man1/openssl-speed/
