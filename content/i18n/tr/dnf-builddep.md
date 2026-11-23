# dnf builddep

Belirli bir paketi derlemek için gerekli bağımlılıkları kurmak için kullanılır. Varsayılan olarak `dnf` ile gelmez, `dnf-plugins-core` ile desteklenir.

## Usage
- `dnf builddep yol/tanım.spec` : belirli bir paket için bağımlılıkları kur
- `dnf builddep --skip-unavailable yol/tanım.spec` : belirli bir paket için bağımlılıkları kur, mevcut olmayanları yok say
- `dnf builddep -D 'ifade'` : RPM makrosunu belirli bir ifadeye ayarla
- `dnf builddep --spec argüman` : `.spec` dosya yolu için bir argüman tanımla
- `dnf builddep --srpm argüman` : `.rpm` dosya yolu için bir argüman tanımla
- `dnf builddep --help-cmd` : yardımı görüntüle

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/builddep.html
