# npm

JavaScript ve Node.js paket yöneticisi. Node.js projelerini ve modül bağımlılıklarını yönet. `install`, `run` gibi bazı alt komutların kendi kullanım dokümantasyonu vardır.

## Usage
- `npm init -y` : varsayılan değerlerle bir `package.json` dosyası oluştur (etkileşimli yapmak için `--yes` kullanma)
- `npm i` : bağımlılık olarak `package.json` dosyasında listelenen tüm paketleri indir
- `npm i paket_adı@sürüm` : bir paketin belirli bir sürümünü indir ve `package.json` dosyasındaki bağımlılık listesine ekle
- `npm i paket_adı -D` : bir paketin son sürümünü indir ve `package.json` dosyasındaki geliştirme bağımlılıkları listesine ekle
- `npm i paket_adı -g` : bir paketin son sürümünü indir ve global olarak kur (kurulum konumunu `npm config set prefix` ile ayarla)
- `npm r paket_adı` : bir paketi kaldır ve `package.json` dosyasındaki bağımlılık listesinden çıkar
- `npm ls` : yerel olarak kurulu tüm bağımlılıkları listele
- `npm ls -g --depth 0` : global olarak kurulu tüm üst düzey paketleri listele

## Links
- More information: https://docs.npmjs.com/cli/npm/
