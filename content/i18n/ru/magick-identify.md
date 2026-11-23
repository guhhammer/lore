# magick identify

Описывать формат и характеристики файлов изображений.

## Usage
- `magick identify путь/к/изображению` : описать формат и основные характеристики изображения
- `magick identify -verbose путь/к/изображению` : описать формат и подробные характеристики изображения
- `magick identify -format "%f,%w,%h\n" *.jpg >путь/к/списку_файлов.csv` : собрать размеры всех JPEG-файлов в текущем каталоге и сохранить их в CSV-файл

## Links
- More information: https://imagemagick.org/script/identify.php
