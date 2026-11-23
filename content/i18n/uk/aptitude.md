# aptitude

Утиліта керування пакетами Debian і Ubuntu.

## Usage
- `sudo aptitude update` : синхронізувати список доступних пакетів і версій. Це слід запустити спочатку, перш ніж запускати наступні команди aptitude
- `sudo aptitude install пакет` : встановити новий пакет і його залежності
- `aptitude search пакет` : шукати пакет
- `aptitude search '?installed(пакет)'` : шукати встановлений пакет (`?installed` це термін пошуку aptitude)
- `sudo aptitude remove пакет` : видалити пакет і всі залежні від нього пакети
- `sudo aptitude upgrade` : оновити встановлені пакети до найновіших доступних версій
- `sudo aptitude full-upgrade` : оновити встановлені пакети (як `aptitude upgrade`) включно з видаленням застарілих пакетів і встановленням додаткових, щоб відповідати новим залежностям пакетів
- `sudo aptitude hold '?installed(пакет)'` : затримати встановлений пакет, щоб уникнути його автоматичного оновлення

## Links
- More information: https://manned.org/aptitude
