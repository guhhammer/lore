# wget

Скачивать файлы из интернета. Поддерживает HTTP, HTTPS и FTP.

## Usage
- `wget https://example.com/foo` : скачать содержимое URL в файл (в данном случае с именем "foo")
- `wget -O путь/к/файлу https://example.com/foo` : скачать содержимое URL в файл (в данном случае с именем "bar")
- `wget -pkw 3 https://example.com/some_page.html` : скачать одну веб-страницу и все её ресурсы с интервалом в 3 секунды между запросами (скрипты, таблицы стилей, изображения и т.д.)
- `wget -mnp https://example.com/some_path/` : скачать все перечисленные файлы в каталоге и его подкаталогах (не скачивает встроенные элементы страницы)
- `wget --limit-rate 300k -t 100 https://example.com/some_path/` : ограничить скорость скачивания и количество попыток подключения
- `wget --user имя_пользователя --password пароль https://example.com` : скачать файл с HTTP-сервера, используя базовую аутентификацию (также работает для FTP)
- `wget -c https://example.com` : продолжить незавершённое скачивание
- `wget -P путь/к/каталогу -i путь/к/URLs.txt` : скачать все URL, сохранённые в текстовом файле, в указанный каталог

## Links
- More information: https://www.gnu.org/software/wget/manual/wget.html
