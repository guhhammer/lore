# usermod

Bir kullanıcı hesabını düzenleyin.

## Usage
- `sudo usermod -l yeni_kullanıcı_adı kullanıcı_adı` : kullanıcı adını değiştirin
- `sudo usermod -u id kullanıcı_adı` : kullanıcı ID'sini değiştirin
- `sudo usermod -s kabuk/yolu kullanıcı_adı` : kullanıcı kabuğunu değiştirin
- `sudo usermod -aG grup1,grup2,... kullanıcı_adı` : kullanıcıyı ek gruplara ekleyin (boşluk olmamasına dikkat edin)
- `sudo usermod -rG grup1,grup2,... kullanıcı_adı` : bir gruptan belirtilen kullanıcıyı kaldırın
- `sudo usermod -m -d yeni_ev/dizin/yolu kullanıcı_adı` : kullanıcı ev dizinini değiştirin
- `sudo usermod -L kullanıcı_adı` : hesabı kilitleyin
- `sudo usermod -U kullanıcı_adı` : hesabın kilidini açın

## Links
- More information: https://manned.org/usermod
