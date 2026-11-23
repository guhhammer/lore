# git config

Git depoları için yazılan kişisel konfigürasyon seçeneklerini yönet. Bu konfigürasyonlar lokal (mevcut depo için) veya evrensel (mevcut kullanıcı için) olabilir.

## Usage
- `git config --global user.name|user.email "Adınız Soyadınız|eposta@ornek.com"` : i̇smini veya e-postanı evrensel olarak belirle (bu bilgi bir depoya commit'lemek için gereklidir ve tüm commit'lere eklenir)
- `git config --local|global|system -l --show-origin` : lokal, evrensel veya sistem konfigürasyon kayıtlarını sırala ve bulundukları dosyayı göster
- `git config --global alias.unstage "reset HEAD --"` : belirtilen bir konfigürasyon kaydının evrensel değerini belirle (bu örnekte bir takma ad)
- `git config alias.unstage` : belirtilen bir konfigürasyon kaydının değerini öğren
- `git unstage` : bir takma adı kullan
- `git config --global --unset alias.unstage` : evrensel bir konfigürasyon kaydını varsayılan değerine geri al
- `git config -e` : mevcut depodaki Git konfigürasyonunu (`.git/config`) varsayılan metin düzenleyici ile düzenle
- `git config --global -e` : evrensel Git konfigürasyonunu (varsayılan olarak `~/.gitconfig`, böyle bir dosya varsa `$XDG_CONFIG_HOME/git/config`) varsayılan metin düzenleyici ile düzenle

## Links
- More information: https://git-scm.com/docs/git-config
