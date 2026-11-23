# hostnamectl

Wyświetl lub ustaw nazwę hosta tego komputera.

## Usage
- `hostnamectl` : wyświetl nazwę hosta tego komputera
- `sudo hostnamectl set-hostname "nazwa_hosta"` : ustaw nazwę hosta tego komputera
- `sudo hostnamectl set-hostname --static "nazwa_hosta.example.com" && sudo hostnamectl set-hostname --pretty "nazwa_hosta"` : ustaw ładną nazwę hosta tego komputera
- `sudo hostnamectl set-hostname --pretty ""` : zresetuj nazwę hosta do jej domyślnej wartości

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/hostnamectl.html
