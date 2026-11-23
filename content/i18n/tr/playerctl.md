# playerctl

Medya oynatıcılarını MPRIS aracılığıyla kumanda edin.

## Usage
- `playerctl play-pause` : oynatmayı aç/kapat
- `playerctl next` : bir sonraki medyaya geçin
- `playerctl previous` : bir önceki medyaya geçin
- `playerctl -l` : tüm oynatıcıları listeleyin
- `playerctl -p oynatıcı_ismi play-pause|next|previous|...` : belirtilen oynatıcıya komut verin
- `playerctl -a play-pause|next|previous|...` : tüm oynatıcılara komut verin
- `playerctl metadata -f "Şuanda oynatılıyor: \{\{artist\}\} - \{\{album\}\} - \{\{title\}\}"` : mevcut medyanın meta verisini görüntüleyin

## Links
- More information: https://github.com/altdesktop/playerctl#using-the-cli
