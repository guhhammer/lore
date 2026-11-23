# dnf install

Устанавливать пакеты в дистрибутивах на базе Red Hat.

## Usage
- `sudo dnf in пакет1 пакет2 ...` : установить пакеты по имени
- `sudo dnf in путь/к/файлу` : установить пакет из локального файла
- `sudo dnf in https://example.com/package.rpm` : установить пакет из интернета
- `sudo dnf in https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm` : добавить репозиторий несвободных пакетов Fedora
- `sudo dnf in epel-release` : добавить репозитории Extra Packages for Enterprise Linux (EPEL)
- `sudo dnf in https://rpms.remirepo.net/enterprise/remi-release-8.rpm` : добавить RPM-репозиторий Remi

## Links
- More information: https://dnf.readthedocs.io/en/latest/command_ref.html#install-examples
