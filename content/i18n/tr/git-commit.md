# git commit

Depoya dosya commit'le.

## Usage
- `git commit` : bir düzenleyici aç, mesajı yaz ve sahnelenmiş dosyaları depoya commit'le
- `git commit -m "mesaj"` : sahnelenmiş dosyaları belirtilen mesaj ile depoya commit'le
- `git commit -F yol/commit_mesajı_dosyası` : sahnelenmiş dosyaları, mesajı bir dosyadan okuyarak commit'le
- `git commit -a -m "mesaj"` : değiştirilmiş ve silinmiş tüm dosyaları otomatik olarak sahnele ve commit'le
- `git commit -S anahtar_kimliği -m "mesaj"` : sahnelenmiş dosyaları commit'le ve belirtilen GPG anahtarı ile imzala (`anahtar_kimliği` belirtilmezse yapılandırma dosyasında tanımlı olan kullanılır)
- `git commit --amend` : şu anda sahnelenmiş değişiklikleri ekleyerek son commit'i güncelle (commit'in hash'i değişir) ve mesajı değiştirmek için bir düzenleyici aç
- `git commit yol/dosya1 yol/dosya2 ...` : yalnızca belirtilmiş (halihazırda sahnelenmiş) dosyaları commit'le
- `git commit -m "mesaj" -m "açıklama"` : belirtilen mesaj ve açıklama ile bir commit oluştur

## Links
- More information: https://git-scm.com/docs/git-commit
