# chromium

Веб-браузер с открытым исходным кодом, в основном разрабатываемый и поддерживаемый Google. Примечание: Возможно, вам потребуется заменить команду `chromium` на желаемый веб-браузер, такой как `brave`, `google-chrome`, `microsoft-edge`/`msedge`, `opera` или `vivaldi`.

## Usage
- `chromium https://example.com|путь\к\файлу.html` : открыть указанный URL-адрес или файл
- `chromium --incognito|msedge --inprivate example.com` : открыть в режиме инкогнито (используйте `--inprivate` для Microsoft Edge)
- `chromium --new-window example.com` : открыть в новом окне
- `chromium --app https://example.com` : открыть в режиме приложения (без панелей инструментов, адресной строки, кнопок и т.д.)
- `chromium --proxy-server "socks5://hostname:66" example.com` : использовать прокси-сервер
- `chromium --user-data-dir путь\к\каталогу` : открыть с пользовательским каталогом профиля
- `chromium --user-data-dir путь\к\каталогу --disable-web-security` : открыть без проверки CORS (полезно для тестирования API)
- `chromium --auto-open-devtools-for-tabs` : открыть с окном DevTools для каждой новой вкладки

## Links
- More information: https://www.chromium.org/developers/how-tos/run-chromium-with-flags/
