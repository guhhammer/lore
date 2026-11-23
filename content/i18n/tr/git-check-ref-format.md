# git check-ref-format

Girilen referans isminin kabul edilebilir olup olmadığını kontrol eder, ve eğer kabul edilemezse sıfır olmayan bir çıktı verir.

## Usage
- `git check-ref-format refs/head/referans_ismi` : belirtilen referans ismini biçimini kontrol et
- `git check-ref-format --branch @{-1}` : son kontrol edilen dalın ismini göster
- `git check-ref-format --normalize refs/head/referans_ismi` : bir referans ismi dosyasını normalleştir

## Links
- More information: https://git-scm.com/docs/git-check-ref-format
