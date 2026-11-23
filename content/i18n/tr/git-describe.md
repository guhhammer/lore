# git describe

Bir nesneye varolan referans üzerinden insanlar tarafından okunabilecek biçimde olan bir isim ver.

## Usage
- `git describe` : mevcut commit için (en son eklenmiş etiket, ilave commit'lerin sayısı ve kısaltılmış commit değerini içeren) özel bir isim oluştur
- `git describe --abbrev=4` : kısaltılmış commit değeri için 4 haneli bir isim oluştur
- `git describe --all` : etiket referans yolu ile bir isim oluştur
- `git describe v1.0.0` : bir Git etiketini açıkla
- `git describe dal_ismi` : belirtilen daldaki son commit için bir isim oluştur

## Links
- More information: https://git-scm.com/docs/git-describe
