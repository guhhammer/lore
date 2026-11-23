# conda

Herhangi bir programlama dili için paketleri, bağımlılıkları ve ortamları yönet. `create` gibi bazı alt komutların kendi kullanım dokümantasyonu vardır.

## Usage
- `conda create -n ortam_adı python=3.9 matplotlib` : yeni bir ortam oluştur ve belirtilen paketleri bu ortama kur
- `conda info -e` : tüm ortamları listele
- `conda activate ortam_adı` : bir ortamı etkinleştir
- `conda deactivate` : bir ortamı devre dışı bırak
- `conda remove -n ortam_adı --all` : bir ortamı (tüm paketleri kaldırarak) sil
- `conda install python=3.4 numpy` : geçerli ortama paket kur
- `conda list` : geçerli ortamda kurulu paketleri listele
- `conda clean -a` : kullanılmayan paketleri ve önbellekleri sil

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/index.html
