# fd

Dosya sisteminde girdileri bul.

## Usage
- `fd "normal_ifade"` : geçerli dizinde belirli bir desenle eşleşen dosyaları özyinelemeli olarak bul
- `fd "normal_ifade" yol/dizin` : belirli bir dizindeki dosyaları bul
- `fd -e txt` : belirli bir uzantıya sahip dosyaları bul
- `fd "normal_ifade" -t d` : yalnızca belirli bir desenle eşleşen dizinleri bul
- `fd "normal_ifade" -HI` : yok sayılan ve gizli dosyaları da aramaya dahil et
- `fd "normal_ifade" -E glob` : belirli bir `glob` desenine uyan dosyaları hariç tut
- `fd "normal_ifade" -x komut` : döndürülen her arama sonucu üzerinde bir komut çalıştır
- `fd "normal_ifade" -d 1` : yalnızca geçerli dizindeki dosyaları bul

## Links
- More information: https://github.com/sharkdp/fd#command-line-options
