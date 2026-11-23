# magick import

Захватывать часть или весь экран X-сервера и сохранять изображение в файл.

## Usage
- `magick import -window root путь/к/выходному_файлу.ps` : захватить весь экран X-сервера в файл PostScript
- `magick import -window root -display удалённый_хост:экран.дисплей путь/к/выходному_файлу.png` : захватить содержимое экрана удалённого X-сервера в PNG-изображение
- `magick import -window идентификатор_окна путь/к/выходному_файлу.jpg` : захватить конкретное окно по его идентификатору, отображаемому `xwininfo`, в JPEG-изображение

## Links
- More information: https://imagemagick.org/script/import.php
