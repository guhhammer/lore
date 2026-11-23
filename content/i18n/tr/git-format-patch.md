# git format-patch

`.patch` dosyaları oluştur. Commit'leri e-posta olarak gönderirken işe yarar.

## Usage
- `git format-patch origin` : gönderilmemiş tüm commit'ler için otomatik olarak adlandırılan bir `.patch` dosyası oluştur
- `git format-patch revizyon_1..revizyon_2` : `stdout`'daki belirtilen 2 revizyon arasındaki tüm commit'ler için bir `.patch` dosyası oluştur
- `git format-patch -3` : son 3 commit için bit `.patch` dosyası oluştur

## Links
- More information: https://git-scm.com/docs/git-format-patch
