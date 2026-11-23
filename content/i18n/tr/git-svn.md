# git svn

Bir alt sürüm deposu ve Git arasında çift yönlü operasyon.

## Usage
- `git svn clone https://ornek.com/altsürüm_deposu yerel_dizin` : bit SVN deposunu klonla
- `git svn clone -r 1234:HEAD https://svn.ornek.net/altsürüm/depo yerel_dizin` : bir SVN deposunu belirtilen düzenleme numarasından başlayarak klonla
- `git svn rebase` : uzak SVN deposundan yerel klonu güncelle
- `git svn fetch` : git HEAD'i değiştirmeden uzak SVN deposundan güncellemeleri çek
- `git svn commit` : SVN deposuna geri commit'le

## Links
- More information: https://git-scm.com/docs/git-svn
