# git bisect

Bug taşıyan commit'i bulmak için ikili arama kullan. Git otomatik olarak commit çizelgesi içinde oradan oraya atlayarak yaramaz commit'i saptar.

## Usage
- `git bisect start kötü_commit iyi_commit` : buglı bilinen bir commit'i ve (genelde eski olan) iyi bir commit'i belirterek ikiye bölme işlemini başlat
- `git bisect good|bad` : `git bisect`'in seçtiği her commit'i, mevcut soruna sebep olup olmadıklarını test ettikten sonra "bad" (kötü) veya "good" (iyi) olarak işaretle
- `git bisect reset` : i̇kiye bölme işlemini bitir ve önceki dala geri dön
- `git bisect skip` : i̇kiye bölme işlemi sırasında bir commit'i atla (ör. başka bir sorun yüzünden testleri geçemeyen bir commit)
- `git bisect start kötü_commit iyi_commit -- yol/dosya_veya_dizin` : yalnızca belirli bir dosyayı veya dizini değiştiren commit'leri dikkate alarak ikiye bölme işlemini başlat
- `git bisect run yol/test_betiği isteğe_bağlı_betik_argümanları` : i̇kiye bölme işlemini, "iyi" için 0, "kötü" için sıfırdan farklı bir çıkış kodu döndüren bir test betiği kullanarak otomatikleştir
- `git bisect log` : şu ana kadar yapılanların kaydını göster
- `git bisect visualize` : kontrol edilecek kalan aday commit'leri göster

## Links
- More information: https://git-scm.com/docs/git-bisect
