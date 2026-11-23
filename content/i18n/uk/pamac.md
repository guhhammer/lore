# pamac

Утиліта командного рядка для GUI менеджера пакетів pamac. Якщо ви не можете побачити пакети AUR, увімкніть його `/etc/pamac.conf` або в GUI.

## Usage
- `pamac install назва_пакета` : встановити новий пакет
- `pamac remove --orphans назва_пакета` : видалити пакет і його непотрібні залежності (сироти)
- `pamac search назва_пакета` : шукати пакет в базі даних пакетів
- `pamac list --installed` : перелічити встановлені пакети
- `pamac checkupdates` : перевірити наявність оновлень пакетів
- `pamac upgrade` : оновити всі пакети

## Links
- More information: https://wiki.manjaro.org/index.php/Pamac
