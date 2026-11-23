# apt-mark

Утилита для изменения статуса установленных пакетов.

## Usage
- `sudo apt-mark auto пакет` : отметить пакет как автоматически установленный
- `sudo apt-mark hold пакет` : удерживать пакет в его текущей версии и запретить его обновление
- `sudo apt-mark unhold пакет` : перестать удерживать пакет и разрешить его обновление
- `apt-mark showmanual` : показать пакеты, установленные вручную
- `apt-mark showhold` : показать удерживаемые пакеты

## Links
- More information: https://manned.org/apt-mark
