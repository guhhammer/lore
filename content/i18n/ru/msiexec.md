# msiexec

Установка, обновление, восстановление или удаление программ Windows через пакеты установки MSI и MSP.

## Usage
- `msiexec /package путь\к\файлу.msi` : установить программу из MSI-пакета
- `msiexec /package https://example.com/installer.msi` : установить MSI-пакет с веб-сайта
- `msiexec /update путь\к\файлу.msp` : установить MSP-пакет с обновлением (патчем)
- `msiexec /uninstall путь\к\файлу` : удалить программу или обновление, используя соответствующий пакет MSI или MSP
