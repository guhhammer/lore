# xzdiff

Roept `diff` aan op bestanden gecomprimeerd met `xz`, `lzma`, `gzip`, `bzip2`, `lzop` of `zstd`. Alle opgegeven opties worden direct doorgegeven aan `diff`.

## Usage
- `xzdiff pad/naar/bestand1 pad/naar/bestand2` : vergelijk twee bestanden
- `xzdiff --side-by-side pad/naar/bestand1 pad/naar/bestand2` : vergelijk twee bestanden en toon de verschillen naast elkaar
- `xzdiff --brief pad/naar/bestand1 pad/naar/bestand2` : vergelijk twee bestanden en rapporteer alleen dat ze verschillen (geen details over wat er verschilt)
- `xzdiff --report-identical-files pad/naar/bestand1 pad/naar/bestand2` : vergelijk twee bestanden en rapporteer wanneer de bestanden hetzelfde zijn
- `xzdiff --paginate pad/naar/bestand1 pad/naar/bestand2` : vergelijk twee bestanden met gepagineerde resultaten

## Links
- More information: https://manned.org/xzdiff
