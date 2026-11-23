# arjun

Обнаруживать HTTP-параметры веб-приложений.

## Usage
- `arjun -u https://example.com/page.php` : просканировать URL-адрес на наличие GET-параметров
- `arjun -u https://example.com/api -m POST` : просканировать, используя метод POST
- `arjun -u https://example.com -o путь/к/выходному_файлу.json` : сохранить найденные параметры в JSON-файл
- `arjun -u https://example.com -w путь/к/словарю.txt` : использовать пользовательский словарь (wordlist)
- `arjun -u https://example.com -d 2` : увеличить задержку между запросами на указанное количество секунд, чтобы избежать ограничения частоты запросов (rate limiting)

## Links
- More information: https://github.com/s0md3v/Arjun/wiki/Usage
