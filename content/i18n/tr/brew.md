# brew

Homebrew - macOS ve Linux için bir paket yöneticisi. `install` gibi bazı alt komutların kendi kullanım dokümantasyonu vardır.

## Usage
- `brew install formül|cask` : bir formülün (formula) ya da cask'in en son kararlı sürümünü kur
- `brew list` : kurulu tüm formülleri ve cask'leri listele
- `brew upgrade formül|cask` : kurulu bir formülü ya da cask'i yükselt (hiçbiri belirtilmezse kurulu tüm formüller/cask'ler yükseltilir)
- `brew update` : homebrew'un ve tüm formül ve cask'lerin en yeni sürümünü Homebrew kaynak deposundan getir
- `brew outdated` : daha yeni bir sürümü bulunan formülleri ve cask'leri göster
- `brew search metin` : mevcut formülleri (yani paketleri) ve cask'leri (yani macOS'un yerel `.app` paketlerini) ara
- `brew info formül|cask` : bir formül ya da cask hakkında bilgi göster (sürüm, kurulum yolu, bağımlılıklar vb.)
- `brew doctor` : yerel Homebrew kurulumunda olası sorunları kontrol et

## Links
- More information: https://docs.brew.sh/Manpage
