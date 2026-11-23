# git subtree

Proje bağımlılıklarını alt proje olarak yönetmeye yarayan bir araç.

## Usage
- `git subtree add -P dizin/konumu --squash depo_url'si dal_ismi` : bir Git deposunu alt ağaç olarak ekle
- `git subtree pull -P dizin/konumu depo_url'si dal_ismi` : alt ağaç deposunu son commit'ine güncelle
- `git subtree merge -P dizin/konumu --squash depo_url'si dal_ismi` : son alt ağaca kadar olan değişiklikleri alt ağaca commit'le
- `git subtree push -P dizin/konumu depo_url'si dal_ismi` : commit'leri bir alt ağaç deposuna yolla
- `git subtree split -P dizin/konumu depo_url'si -b dal_ismi` : bir alt ağacın geçmişinden yeni bir proje geçmişi dışa aktar

## Links
- More information: https://manned.org/git-subtree
