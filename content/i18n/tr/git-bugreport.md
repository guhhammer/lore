# git bugreport

Sşstem ve kullanıcıdan hata ayıklama bilgisi çeker ve olası bir Git hatasının rapor edilmesi için bu bilgiyi oluşturduğu bir metin dosyasına kaydeder.

## Usage
- `git bugreport` : mevcut dizinde yeni bir hata rapor dosyası oluştur
- `git bugreport -o örnek/dizin` : belirtilen dizinde yeni bir hata rapor dosyası oluştur
- `git bugreport -s %m%d%y` : `strftime` formatında belirtilmiş bir dosya adı ekiyle yeni bir rapor dosyası oluştur

## Links
- More information: https://git-scm.com/docs/git-bugreport
