# dnf config-manager

Fedora tabanlı sistemlerde DNF yapılandırma seçeneklerini ve depoları yönetmek için kullanılır. Varsayılan olarak `dnf` ile gelmez, `dnf-plugins-core` ile desteklenir.

## Usage
- `dnf config-manager --add-repo=repository_url` : bir URL'den depo ekle (ve etkinleştir)
- `dnf config-manager --dump` : mevcut yapılandırma değerlerini yazdır
- `dnf config-manager --enable repository_id` : belirli bir depoyu etkinleştir
- `dnf config-manager --disable repository_id1 repository_id2 ...` : belirtilen depoları devre dışı bırak
- `dnf config-manager --setopt=option=value` : bir depo için yapılandırma seçeneği ayarla
- `dnf config-manager --help-cmd` : yardımı görüntüle

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/config_manager.html
