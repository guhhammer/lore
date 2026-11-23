# git submodule

Alt modülleri incele, güncelle ve yönet.

## Usage
- `git submodule` : mevcut alt modülleri ve her birinin çekilmiş (checkout) olduğu commit'i göster
- `git submodule update --init --recursive` : deponun (`.gitmodules` dosyasında listelenen) alt modüllerini indir
- `git submodule add depo_url'si` : bir Git deposunu alt modül olarak ekle
- `git submodule add depo_url'si yol/dizin` : bir Git deposunu alt modül olarak belirtilen dizinde ekle
- `git submodule update --remote` : tüm alt modülleri son commit'lerine güncelle
- `git submodule set-url alt_modül/konumu yeni_url` : bir alt modülün URL'sini değiştir
- `git submodule deinit alt_modül/konumu` : bir alt modülün kaydını sil (örneğin `git rm` ile depodan kaldırmadan önce)

## Links
- More information: https://git-scm.com/docs/git-submodule
