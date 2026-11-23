# git difftool

Harici diff araçları kullanarak dosya değişimlerini göster. `git diff` ile aynı ayar ve argümanları destekler.

## Usage
- `git difftool --tool-help` : müsait diff araçlarını göster
- `git config --global diff.tool "meld"` : varsayılan diff aracını birleştirmeye ayarla
- `git difftool --staged` : varsayılan diff aracını sahnelenmiş değişiklikleri göstermek için kullan
- `git difftool -t opendiff commit` : verilen commit'den itibaren yapılmış değişiklikleri göstermek için (opendiff) kullan

## Links
- More information: https://git-scm.com/docs/git-difftool
