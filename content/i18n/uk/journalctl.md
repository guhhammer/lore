# journalctl

Запити до журналу systemd.

## Usage
- `journalctl -b -p 3` : показати всі повідомлення з рівнем пріоритету 3 (помилки) від цього завантаження
- `journalctl --vacuum-time 2d` : видалити записи журналу, які старіші за 2 дні
- `journalctl -f` : слідкувати за новими повідомленнями (як `tail -f` для традиційного syslog)
- `journalctl -u блок` : показати всі повідомлення за конкретним блоком
- `journalctl -S now|today|yesterday|tomorrow -U "YYYY-MM-DD HH:MM:SS"` : фільтрувати повідомлення в межах діапазону часу (мітка часу або покажчики місця заповнення, як-от "вчора")
- `journalctl _PID=pid` : показати всі повідомлення за певним процесом
- `journalctl шлях/до/виконуваного_файлу` : показати всі повідомлення за певним виконуваним файлом

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html
