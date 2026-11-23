# pacman-key

pacman'ın anahtar zincirini yönetmek için kullanılan GnuPG sarmalayıcı betiği.

## Usage
- `sudo pacman-key --init` : `pacman` anahtar zincirini başlat
- `sudo pacman-key --populate` : varsayılan Arch Linux anahtarlarını ekle
- `pacman-key -l` : açık anahtar zincirindeki anahtarları listele
- `sudo pacman-key -a yol/anahtar_dosyasi.gpg` : belirtilen anahtarları ekle
- `sudo pacman-key -r "uid|isim|email"` : bir anahtarı anahtar sunucusundan al
- `pacman-key -f "uid|isim|email"` : belirli bir anahtarın parmak izini yazdır
- `sudo pacman-key --lsign-key "uid|isim|email"` : i̇çe aktarılan anahtarı yerel olarak imzala
- `sudo pacman-key -d "uid|isim|email"` : belirli bir anahtarı kaldır

## Links
- More information: https://manned.org/pacman-key
