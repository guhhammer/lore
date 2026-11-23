# hostnamectl

Получение и установка имени хоста компьютера.

## Usage
- `hostnamectl` : получить имя хоста компьютера
- `sudo hostnamectl set-hostname "имя_хоста"` : задать имя хоста компьютера
- `sudo hostnamectl set-hostname --static "имя_хоста.example.com" && sudo hostnamectl set-hostname --pretty "имя_хоста"` : задать красивое (короткое) имя хоста компьютера
- `sudo hostnamectl set-hostname --pretty ""` : сбросить имя хоста компьютера к значению по умолчанию

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/hostnamectl.html
