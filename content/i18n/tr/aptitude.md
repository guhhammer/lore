# aptitude

Debian ve Ubuntu paket yönetim aracı.

## Usage
- `sudo aptitude update` : kullanılabilir paket ve sürüm listesini senkronize et. Bu, herhangi bir aptitude komutunu uygulamadan önce çalıştırılmalıdır
- `sudo aptitude install paket` : yeni bir paket ve onun bağımlılıklarını kur
- `aptitude search paket` : paket ara
- `aptitude search '?installed(paket)'` : i̇ndirilmiş bir paket ara: (`?installed` bir aptitude arama ifadesidir)
- `sudo aptitude remove paket` : bir paket ve onun bağımlılıklarını kaldır
- `sudo aptitude upgrade` : yüklü paketleri son kullanılabilir sürümlerine yükselt
- `sudo aptitude full-upgrade` : yüklü paketleri yükle (`aptitude upgrade` gibi), gereksizleri sil ve yeni bağımlılıkları karşılamak üzere ek paketler kur
- `sudo aptitude hold '?installed(paket)'` : bir paketin otomatik yükseltilmesini engellemek için onu beklemede tut

## Links
- More information: https://manned.org/aptitude
