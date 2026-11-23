# git clone

Varolan bir dizini klonla.

## Usage
- `git clone uzak_bağlantıdaki_depo` : varolan bir depoyu klonla
- `git clone uzak_bağlantıdaki_depo örnek/dizin` : varolan bir depoyu velirtilen dizine klonla
- `git clone --recursive uzak_bağlantıdaki_depo` : varolan bir depo ve onun alt modüllerini klonla
- `git clone -l örnek/yerel/depo` : yerel bir depoyu klonla
- `git clone -q uzak_bağlantıdaki_depo` : sessizce klonla
- `git clone --depth 10 uzak_bağlantıdaki_depo` : yalnızca en yeni 10 commit'i çekerek varolan bir depoyu klonla (zaman tasarrufu açısından yararlıdır)
- `git clone --branch isim --single-branch uzak_bağlantıdaki_depo` : yalnızca belirtilen bir dalı çekerek varolan bir depoyu klonla

## Links
- More information: https://git-scm.com/docs/git-clone
