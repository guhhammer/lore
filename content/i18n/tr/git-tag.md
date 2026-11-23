# git tag

Etiketleri oluştur, sırala, sil veya doğrula. Bir etiket, belirtilmiş bir commit'e bağlı statik bir referanstır.

## Usage
- `git tag` : tüm etiketleri sırala
- `git tag etiket_ismi` : belirtilen isim ile mevcut commit'e bağlı bir etiket yarat
- `git tag etiket_ismi commit` : belirtilen isim ile belirtilen commit'e bağlı bir etiket yarat
- `git tag etiket_ismi -m etiket_mesajı` : belirtilen mesaja sahip açıklamalı bir etiket yarat
- `git tag -d etiket_ismi` : belirtilen isimdeki etiketi sil
- `git fetch -t` : ana projeden güncellenmiş etiketleri al
- `git tag --contains commit` : belirtilen commit'i içeren/içermiş tüm etiketleri sırala

## Links
- More information: https://git-scm.com/docs/git-tag
