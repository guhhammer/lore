# git apply

İndeks veya dosyalara yama uygula.

## Usage
- `git apply -v örnek/dosya` : yamalanan dosyalarla ilgili mesajları yazdır
- `git apply --index örnek/dosya` : yamalanan dosyaları indekse uygula ve ekle
- `curl -L https://ornek.com/dosya.patch | git apply` : uzak yama dosyası uygula
- `git apply --stat --apply örnek/dosya` : çıktı için fark statistiği çıkar ve yamayı uygula
- `git apply -R örnek/dosya` : yamayı tersten uygula
- `git apply --cached örnek/dosya` : yama sonucunu çalışan ağacı değiştirmeden indekste sakla

## Links
- More information: https://git-scm.com/docs/git-apply
