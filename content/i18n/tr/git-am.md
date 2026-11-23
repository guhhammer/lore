# git am

Yama dosyalarını uygula ve commit oluştur. E-posta ile commit alırken faydalıdır.

## Usage
- `git am örnek/yama.patch` : yerel bir yama dosyasındaki değişiklikleri uygula ve commit oluştur
- `curl -L https://example.com/yama.patch | git am` : uzak bir yama dosyasındaki değişiklikleri uygula ve commit oluştur
- `git am --abort` : yama dosyası uygulama işlemini durdur
- `git am --reject örnek/yama.patch` : yama dosyasının uygulanabilen kısmını uygula, uygulanamayan parçaları `.rej` dosyalarına kaydet

## Links
- More information: https://git-scm.com/docs/git-am
