# git blame

Her satırdaki dosyanın yanında en son commit değeri ve yazarını göster.

## Usage
- `git blame yol/dosya` : bir dosyayı, her satırında son commit değeri ve yazarı bulunacak şekilde göster
- `git blame -e yol/dosya` : bir dosyayı, her satırında son commit değeri ve yazarının e-postası bulunacak şekilde göster
- `git blame commit yol/dosya` : bir dosyayı, belirli bir commit'teki haliyle her satırında yazar bilgisi bulunacak şekilde göster
- `git blame commit~ yol/dosya` : bir dosyayı, belirli bir commit'ten önceki haliyle her satırında yazar bilgisi bulunacak şekilde göster
- `git blame -L 123 yol/dosya` : bir dosyayı, belirli bir satırdan başlayarak her satırında yazar bilgisi bulunacak şekilde göster
- `git blame -L başlangıç_satırı,bitiş_satırı yol/dosya` : bir dosyanın belirli bir satır aralığını yazar bilgisiyle göster
- `git blame -L '/metin/',+10 yol/dosya` : bir dosyanın, verilen metinle eşleşen ilk satırdan başlayarak 10 satırını yazar bilgisiyle göster
- `git blame -w -C -C -C yol/dosya` : bir dosyayı boşlukları ve satır taşımalarını yok sayarak yazar bilgisiyle göster

## Links
- More information: https://git-scm.com/docs/git-blame
