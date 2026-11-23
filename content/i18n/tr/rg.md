# rg

Ripgrep, yinelemeli satır-odaklı bir CLI arama aracıdır. Grep'e daha hızlı bir alternatif olmayı hedefler.

## Usage
- `rg normal_ifade` : normal bir ifade için geçerli dizini yinelemeli olarak ara
- `rg normal_ifade yol/dosya_veya_dizin` : bir dosyada veya dizinde normal bir ifadeyi yinelemeli olarak ara
- `rg -F -- dize` : değişmez bir dize desenini ara
- `rg -. --no-ignore normal_ifade` : gizli dosyaları ve `.gitignore` içinde listelenen girdileri de dahil et
- `rg normal_ifade -g 'dosya_adı_glob_deseni'` : yalnızca bir `glob` desenine uyan dosyalarda ara (örn. `README.*`; hariç tutmak için `!dosya_adı_deseni` kullan)
- `rg --files | rg --passthru normal_ifade` : geçerli dizindeki dosya adlarını yinelemeli olarak listele ve bir desenle eşleşenleri vurgula
- `rg -l normal_ifade` : yalnızca eşleşen dosyaları listele (diğer komutlara yönlendirirken kullanışlıdır)
- `rg -v normal_ifade` : desenle eşleşmeyen satırları göster

## Links
- More information: https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md
