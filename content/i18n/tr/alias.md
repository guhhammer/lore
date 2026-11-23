# alias

Takma adlar/kısayollar (bir komut dizesi ile değiştirilen sözcükler) oluşturur. Kısayollar, kabuğun yapılandırma dosyasında (örneğin `~/.bashrc`) tanımlanmadığı sürece geçerli kabuk oturumuyla birlikte sona erer.

## Usage
- `alias` : tüm kısayolları listele
- `alias kelime="komut"` : genel bir kısayol oluştur
- `alias kelime` : bir kısayolun verildigi komutu göster
- `unalias kelime` : bir kısayolu kaldır
- `alias rm="rm --interactive"` : `rm`'yi interaktif bir komuta dönüştür
- `alias la="ls --all"` : `la`'yi `ls --all` için kısayol olarak oluştur

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
