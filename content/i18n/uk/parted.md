# parted

Програма для роботи з розділами.

## Usage
- `sudo parted --list` : перелічити розділи на всіх блокових пристроях
- `sudo parted /dev/sdX` : запустити інтерактивний режим із вибраним диском
- `sudo parted --script /dev/sdX mklabel aix|amiga|bsd|dvh|gpt|loop|mac|msdos|pc98|sun` : створити нову таблицю розділів указаного типу міток
- `print` : показати інформацію про розділ в інтерактивному режимі
- `select /dev/sdX` : вибрати диск в інтерактивному режимі
- `mkpart primary|logical|extended btrfs|ext2|ext3|ext4|fat16|fat32|hfs|hfs+|linux-swap|ntfs|reiserfs|udf|xfs 0% 16G` : створити розділ на 16 ГБ із зазначеною файловою системою в інтерактивному режимі
- `resizepart /dev/sdXN кінцева_позиція_розділу` : змінити розмір розділу в інтерактивному режимі
- `rm /dev/sdXN` : видалити розділ в інтерактивному режимі

## Links
- More information: https://www.gnu.org/software/parted/manual/parted.html#Invoking-Parted
