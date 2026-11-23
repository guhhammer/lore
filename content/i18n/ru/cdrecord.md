# cdrecord

Записывать данные на CD или DVD. Некоторые вызовы cdrecord могут привести к деструктивным действиям, таким как стирание всех данных на диске.

## Usage
- `cdrecord --devices` : показать оптические приводы, доступные для `cdrecord`
- `cdrecord dev=/dev/optical_drive -audio track*.cdaudio` : записать ("burn") диск только с аудио
- `cdrecord -eject dev=/dev/optical_drive -data file.iso` : записать файл на диск с извлечением диска по завершении (некоторые рекордеры требуют этого)
- `cdrecord -tao dev=/dev/optical_drive -data file.iso` : записать файл на диск в оптическом приводе с возможностью записи на несколько дисков подряд

## Links
- More information: https://manned.org/cdrecord
