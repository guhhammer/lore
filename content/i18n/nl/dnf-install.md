# dnf install

Installeer pakketten op Red Hat-gebaseerde distributies.

## Usage
- `sudo dnf in pakket1 pakket2 ...` : installeer pakketten op naam
- `sudo dnf in pad/naar/bestand` : installeer een pakket vanuit een lokaal bestand
- `sudo dnf in https://example.com/pakket.rpm` : installeer een pakket vanaf het internet
- `sudo dnf in https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm` : voeg de Fedora non-free-pakketrepository toe
- `sudo dnf in epel-release` : voeg de Extra Packages for Enterprise Linux (EPEL) repositories toe
- `sudo dnf in https://rpms.remirepo.net/enterprise/remi-release-8.rpm` : voeg Remi's RPM repository toe

## Links
- More information: https://dnf.readthedocs.io/en/latest/command_ref.html#install-examples
