# xcodes runtimes

Atur pemasangan runtime Simulator yang tersedia bagi aplikasi Xcode.

## Usage
- `xcodes runtimes --include-betas` : tampilkan seluruh Simulator yang tersedia bagi aplikasi Xcode
- `xcodes runtimes download nama_runtime` : unduh sebuah runtime Simulator
- `xcodes runtimes install nama_runtime` : unduh dan pasang sebuah runtime Simulator
- `xcodes runtimes download|install "iOS|watchOS|tvOS|visionOS nama_runtime"` : unduh/pasang runtime Simulator untuk iOS/watchOS/tvOS/visionOS versi spesifik (nama harus ditulis sebagai case-sensitive)
- `xcodes runtimes download|install nama_runtime --directory jalan/menuju/direktori` : atur lokasi penyimpanan arsip runtime yang akan diunduh (nilai default: `~/Downloads`)
- `xcodes runtimes install nama_runtime --keep-archive` : jangan hapus arsip runtime Simulator setelah pemasangan selesai

## Links
- More information: https://github.com/xcodesorg/xcodes#commands
