# dnf

RHEL, Fedora ve CentOS için paket yönetim aracı (yum'un yerini alır). `group` ve `config-manager` gibi bazı alt komutların kendi kullanım dökümantasyonu vardır.

## Usage
- `sudo dnf up` : kurulu paketleri mevcut en yeni sürümlere yükselt
- `dnf se anahtar_kelime1 anahtar_kelime2 ...` : paketleri anahtar kelimelerle ara
- `dnf if paket` : bir paket hakkında detaylı bilgi göster
- `sudo dnf in paket1 paket2 ...` : yeni bir paket kur (`--assumeyes` ile tüm onayları otomatik kabul edebilirsin)
- `sudo dnf rm paket1 paket2 ...` : bir paketi kaldır
- `dnf ls --installed` : kurulu paketleri listele
- `dnf wp komut` : belirli bir komutu sağlayan paketleri bul
- `dnf hist` : geçmiş tüm işlemleri görüntüle

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
