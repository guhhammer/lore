# journalctl

Запрашивать записи из журнала systemd.

## Usage
- `journalctl -n n -f` : показать последние `n` строк и отслеживать новые сообщения (аналогично `tail --follow` для традиционного syslog)
- `journalctl -b -1 -p 3` : показать все сообщения с уровнем приоритета 3 (ошибки) с предпоследней загрузки
- `journalctl -u юнит` : показать все сообщения от определённого юнита
- `journalctl _SYSTEMD_INVOCATION_ID=$(systemctl show --value --property=InvocationID юнит)` : показать логи для указанного юнита с момента его последнего запуска
- `journalctl -S now|today|yesterday|tomorrow|... -U "ГГГГ-ММ-ДД ЧЧ:ММ:СС"` : отфильтровать сообщения по временному диапазону (метка времени или заполнители, такие как "yesterday")
- `journalctl _PID=pid` : показать все сообщения от определённого процесса
- `journalctl путь/к/исполняемому_файлу` : показать все сообщения от определённого исполняемого файла
- `journalctl --vacuum-time 2d` : удалить записи журнала старше 2 дней

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html
