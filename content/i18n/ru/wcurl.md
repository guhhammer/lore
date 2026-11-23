# wcurl

Простая обёртка над `curl` для удобной загрузки файлов.

## Usage
- `wcurl https://example.com/index.html` : скачать содержимое URL в файл, указанный в URL (в данном случае `index.html`)
- `wcurl -o путь/к/файлу https://example.com/index.html` : скачать содержимое URL в файл с указанным именем
- `wcurl --curl-options "--progress-bar --http2" https://example.com/index.html` : скачать содержимое URL с индикатором прогресса и использованием HTTP/2 по умолчанию
- `wcurl --curl-options "--clobber --continue-at -" https://example.com/index.html` : возобновить прерванную загрузку

## Links
- More information: https://curl.se/wcurl/manual.html
