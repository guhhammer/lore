# nodemon

Dosyalar değiştirildiğinde Node uygulamasını yeniden başlatmak için dosyaları izler.

## Usage
- `nodemon dosya/yolu/dosya.js` : verilen dosyayı çalıştırır ve belirtilen dosyadaki değişiklikleri izler
- `rs` : [Etkileşimli] Nodemon'u el ile yeniden başlatır
- `nodemon -i dosya/yolu/dosya_veya_dizin` : belirli dosyaları göz ardı eder
- `nodemon dosya/yolu/dosya.js argümanlar` : node uygulamasına argümanlar geçirir
- `nodemon argümanlar dosya/yolu/dosya.js` : zaten Nodemon argümanları değillerse argümanları Node'un kendisine geçirir (örneğin: `--inspect`)
- `nodemon -x "komut seçenekler" dosya/yolu/komut` : node olmayan bir komut dosyasını çalıştırır
- `nodemon -x "python seçenekler" dosya/yolu/dosya.py` : python kodunu çalıştırır

## Links
- More information: https://github.com/remy/nodemon/tree/main/doc/cli
