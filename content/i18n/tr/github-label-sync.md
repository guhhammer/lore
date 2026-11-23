# github-label-sync

GitHub etiketlerini senkronize etmeye yarayan komut satırı arayüzü.

## Usage
- `github-label-sync --access-token token depo_ismi` : yerel bir `labels.json` dosyası kullanarak etiketleri senkronize et
- `github-label-sync --access-token token --labels url|örnek/json_dosyası depo_ismi` : belirli bir etiketlenen JSON dosyası kullanarak etiketleri senkronize et
- `github-label-sync --access-token token --dry-run depo_ismi` : programı etiketleri gerçekten senkronize etmeden çalıştır
- `github-label-sync --access-token token --allow-added-labels depo_ismi` : `labels.json` içinde olmayan etiketleri sakla
- `github-label-sync depo_ismi` : `$GITHUB_ACCESS_TOKEN` ortam değişkenini kullanarak senkronize et

## Links
- More information: https://github.com/Financial-Times/github-label-sync
