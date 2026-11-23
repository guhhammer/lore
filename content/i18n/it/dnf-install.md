# dnf install

Installa pacchetti su distribuzioni basate su Red Hat.

## Usage
- `sudo dnf in pacchetto1 pacchetto2 ...` : installa pacchetti per nome
- `sudo dnf in percorso/del/file` : installa un pacchetto da un file locale
- `sudo dnf in https://example.com/package.rpm` : installa un pacchetto da internet
- `sudo dnf in https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm` : aggiunge il repository dei pacchetti non-free di Fedora
- `sudo dnf in epel-release` : aggiunge i repository Extra Packages for Enterprise Linux (EPEL)
- `sudo dnf in https://rpms.remirepo.net/enterprise/remi-release-8.rpm` : aggiunge il repository RPM di Remi

## Links
- More information: https://dnf.readthedocs.io/en/latest/command_ref.html#install-examples
