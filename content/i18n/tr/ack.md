# ack

Geliştiriciler için optimize edilmiş, `grep` benzeri bir arama aracı.

## Usage
- `ack "aranacak_desen"` : geçerli dizinde özyinelemeli olarak bir dize veya `regex` içeren dosyaları ara
- `ack -i "aranacak_desen"` : büyük/küçük harf duyarsız bir desen ara
- `ack -o "aranacak_desen"` : bir desenle eşleşen satırları ara, sadece eşleşen metni yazdır (satırın geri kalanını değil)
- `ack -t ruby "aranacak_desen"` : aramayı belirli bir dosya türüyle sınırla
- `ack -t noruby "aranacak_desen"` : belirli bir dosya türündeki dosyalarda arama yapma
- `ack -c -h "aranacak_desen"` : bulunan toplam eşleşme sayısını say
- `ack -c -l "aranacak_desen"` : sadece dosya adlarını ve her dosya için eşleşme sayısını yazdır
- `ack --help-types` : `--type` ile kullanılabilecek tüm değerleri listele

## Links
- More information: https://beyondgrep.com/documentation/
