# jq

Alana özgü bir dil (DSL) kullanan JSON işlemcisi.

## Usage
- `jq '.' yol/dosya.json` : yalnızca `jq` ikili dosyasını kullanarak belirli bir ifadeyi çalıştır (renkli ve biçimlendirilmiş JSON çıktısı yazdır)
- `cat yol/dosya.json | jq -f yol/betik.jq` : belirli bir betiği çalıştır
- `cat yol/dosya.json | jq --arg "isim1" "değer1" --arg "isim2" "değer2" ... '. + $ARGS.named'` : belirli argümanlar geçir
- `cat yol/birden_fazla_json_dosya_*.json | jq '{yeniAnahtar1: .anahtar1, yeniAnahtar2: .anahtar2.altAnahtar, ...}'` : birden fazla dosyadaki eski JSON nesnelerinden yeni bir JSON nesnesi oluştur
- `cat yol/dosya.json | jq '.[indeks1], .[indeks2], ...'` : dizideki belirli öğeleri yazdır
- `cat yol/dosya.json | jq '.[]'` : tüm dizi/nesne değerlerini yazdır
- `cat yol/dosya.json | jq '.[] | select((.anahtar1=="değer1") and .anahtar2=="değer2")'` : dizideki nesneleri 2 koşullu bir filtreyle yazdır
- `cat yol/dosya.json | jq '. +|- {"anahtar1": "değer1", "anahtar2": "değer2", ...}'` : belirli anahtarları ekle/kaldır

## Links
- More information: https://jqlang.org/manual/
