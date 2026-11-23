# dnf changelog

Belirli bir paketin değişiklik kayıtlarını görüntülemek için kullanılır. Varsayılan olarak `dnf` ile gelmez, `dnf-plugins-core` ile desteklenir.

## Usage
- `dnf changelog paket` : belirli bir paketin tüm değişiklik kayıtlarını görüntüle
- `dnf changelog --since tarih paket` : belirli bir paketin, belirli bir tarihten sonraki tüm değişiklik kayıtlarını görüntüle
- `dnf changelog --count sayi paket` : belirli bir paketin son `n` değişiklik kaydını görüntüle
- `dnf changelog --upgrades paket` : yalnızca yükseltilebilir paketler için yeni öğeleri göster
- `dnf changelog --help-cmd` : yardımı görüntüle

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/changelog.html
