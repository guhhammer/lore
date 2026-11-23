# dog

Утиліта пошуку DNS. Вона має кольоровий вихід, підтримує протоколи DNS-over-TLS і DNS-over-HTTPS та може видавати JSON.

## Usage
- `dog example.com` : шукає IP-адреси пов'язані з іменем хоста (A records)
- `dog example.com MX` : запитує тип записів MX, пов’язаних із заданим доменним ім’ям
- `dog example.com MX @1.1.1.1` : вкажіть конкретний DNS-сервер для запиту (наприклад, Cloudflare)
- `dog example.com MX @1.1.1.1 -T` : запит через TCP, а не UDP
- `dog -q example.com -t MX -n 1.1.1.1 -T` : запитує тип записів MX, пов’язаних із заданим доменним ім’ям через TCP, використовуючи явні аргументи
- `dog example.com -H @https://cloudflare-dns.com/dns-query` : шукає IP-адреси, пов’язані з іменем хоста (записи A), за допомогою DNS через HTTPS (DoH)

## Links
- More information: https://github.com/ogham/dog#examples
