# git bundle

Cisim ve referansları bir arşive paketle.

## Usage
- `git bundle create örnek/dosyas.bundle dal_ismi` : belirtilmiş bir dalın tüm cisim ve referanslarını içeren bir paket dosyası oluştur
- `git bundle create örnek/dosyas.bundle --all` : tüm dallar için bir paket dosyası oluştur
- `git bundle create örnek/dosya.bundle -5 HEAD` : mevcut daldaki en son 5 commit için bir paket dosyası oluştur
- `git bundle create örnek/dosya.bundle --since=7.days HEAD` : son 7 günü içeren bir paket dosyası oluştur
- `git bundle verify örnek/dosya.bundle` : bir paket dosyasının geçerli olduğunu ve mevcut depoya uygulanabileceğini doğrula
- `git bundle unbundle örnek/dosya.bundle` : bir pakette bulunan referansları sırala
- `git pull örnek/dosya.bundle dal_ismi` : belirtilen dalı paket dosyasından çıkarıp mevcut depoya koy

## Links
- More information: https://git-scm.com/docs/git-bundle
