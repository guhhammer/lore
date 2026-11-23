# git instaweb

gitweb sunucusu başlatmak için yardımcı araç.

## Usage
- `git instaweb --start` : mevcut Git deposu için bir gitweb sunucusu başlat
- `git instaweb --start -l` : yalnızca yerel ağda başlat
- `git instaweb --start -p 1234` : belirtilmiş bir port'da başlat
- `git instaweb --start -d lighttpd|apache2|mongoose|plackup|webrick` : belirtilmiş bir HTTP daemon'u kullan
- `git instaweb --start -b` : ayrıca bir ağ tarayıcısını otomatik olarak başlat
- `git instaweb --stop` : çalışan mevcut gitweb sunucusunu durdur
- `git instaweb --restart` : çalışan mevcut gitweb sunucusunu yeniden başlat

## Links
- More information: https://git-scm.com/docs/git-instaweb
