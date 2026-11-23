# aws accessanalyzer

Analisa dan tinjau ulang kebijakan penggunaan sumber daya untuk melihat potensi risiko keamanan.

## Usage
- `aws accessanalyzer create-analyzer --analyzer-name nama_analyzer --type tipe_analisis --tags daftar_tag` : buat suatu instansi Access Analyzer
- `aws accessanalyzer delete-analyzer --analyzer-arn arn_analyzer` : hapus suatu instansi Access Analyzer
- `aws accessanalyzer get-analyzer --analyzer-arn arn_analyzer` : tampilkan rincian tentang suatu instansi Access Analyzer
- `aws accessanalyzer list-analyzers` : tampilkan daftar seluruh instansi Access Analyzers
- `aws accessanalyzer update-analyzer --analyzer-arn arn_analyzer --tags daftar_tag_baru` : mutakhirkan suatu aturan terhadap instansi Access Analyzer
- `aws accessanalyzer create-archive-rule --analyzer-arn arn_analyzer --rule-name nama_aturan --filter filter` : buat sebuah aturan bagi proses pengarsipan terhadap instansi Access Analyzer
- `aws accessanalyzer delete-archive-rule --analyzer-arn arn_analyzer --rule-name nama_aturan` : hapus suatu aturan pengarsipan bagi instansi Access Analyzer
- `aws accessanalyzer list-archive-rules --analyzer-arn arn_analyzer` : tampilkan daftar seluruh aturan pengarsipan yang berlaku bagi suatu instansi Access Analyzer

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/accessanalyzer/
