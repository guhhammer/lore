# git reset

Mevcut Git HEAD'ini belirtilen duruma sıfırlayarak commit'leri veya değişiklikleri geri al. Eğer bir konum verildiye o konumdaki değişiklikler "geri alınır"; eğer bir commit değeri veya dal verildiyse o commit/dal "geri alınır".

## Usage
- `git reset` : her şeyi geri al
- `git reset dosya(ların)/konumu` : belirtilen dosya(lar)ı geri al
- `git reset -p dosya/konumu` : bir dosyanın kısımlarını geri al
- `git reset HEAD~` : son commit'i, dosya sisteminde yapılan değişiklikleri geri almadan geri al
- `git reset --soft HEAD~2` : son iki commit'i onların indeks'e yaptığı değişiklikleri ekleyerek geri al
- `git reset --hard` : commit'lenmemiş değişiklikleri sahnelenip sahnelenmediklerine bakmaksızın iptal et (sadece sahnelenmemiş değişiklikleri iptal etmek için `git checkout` kullanılır)
- `git reset --hard commit` : depoyu belirtilen commit'e o zamana kadar yapılan değişiklikleri iptal ederek sıfırla

## Links
- More information: https://git-scm.com/docs/git-reset
