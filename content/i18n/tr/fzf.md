# fzf

Komut satırı belirsiz bulucu.

## Usage
- `find dosya/yolu/dizin -type f | fzf` : belirtilen dizindeki tüm dosyalarda `fzf`'yi başlat
- `ps aux | fzf` : çalışan süreçler için `fzf`'yi başlat
- `find dosya/yolu/dizin -type f | fzf -m >dosya/yolu/dosya` : `<Shift Tab>` ile birden çok dosya seç ve bir dosyaya yaz
- `fzf -q "sorgu"` : `fzf`'yi belirli bir sorgu ile başlat
- `fzf -q "^core go$ | rb$ | py$"` : `core` ile başlayan ve `go`, `rb` veya `py` ile biten girişlerde `fzf`'yi başlat
- `fzf -q '!pyc travis'` : PYC ile eşleşmeyen ve Travis'e tam olarak eşleşen girişlerde `fzf`'yi başlat

## Links
- More information: https://github.com/junegunn/fzf#usage
