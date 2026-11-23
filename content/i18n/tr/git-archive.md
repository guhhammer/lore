# git archive

İsimlendirilmiş bir ağaçtan dosyaların arşivini oluştur.

## Usage
- `git archive -v HEAD` : mevcut HEAD'deki içerik ile bir `.tar` arşivi oluştur ve içeriği standart çıktı biçiminde göster
- `git archive -v --format zip HEAD` : mevcut HEAD'deki içerik ile bir zip arşivi oluştur ve içeriği standart çıktı biçiminde göster
- `git archive -v -o örnek/arşiv/dosyası.zip HEAD` : yukarıda yazan madde ile aynı şeyi yap, ama zip arşivini belirtilen dosya olarak yaz
- `git archive -o örnek/arşiv/dosyası.tar dal_ismi` : belirtilmiş bir daldaki son commitlerin içeriğinden bir `.tar` arşivi oluştur
- `git archive -o örnek/arşiv/dosyası.tar HEAD:örnek/dizin` : belirtilmiş bir dizindeki içeriklerden `.tar` arşivi oluştur
- `git archive -o örnek/arşiv/dosyası.tar --prefix başlarına/yol/eklenecek/dosyalar/ HEAD` : bir takım dosyayı belirtilmiş bir dizinin içinde arşivlemek için başlarına yol ekle

## Links
- More information: https://git-scm.com/docs/git-archive
