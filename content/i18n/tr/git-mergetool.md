# git mergetool

Birleştirme sırasında yaşanan karışıklıkları çözmek için karışıklık çözücü araçları çalıştırır.

## Usage
- `git mergetool` : karışıklıkları çözmek için varsayılan birleştirme aracını başlat
- `git mergetool --tool-help` : kullanılabilir birleştirme araçlarını sırala
- `git mergetool -t araç_ismi` : belirtilen birleştirme aracını başlat
- `git mergetool -y` : her birleştirme aracı çağrılışında harekete geçme
- `git mergetool -g` : özellikle grafiksel (GUI) birleştirme aracını kullan (merge.guitool değişkenine göz at)
- `git mergetool --no-gui` : özellikle normal birleştirme aracını kullan (merge.guitool değişkenine göz at)

## Links
- More information: https://git-scm.com/docs/git-mergetool
