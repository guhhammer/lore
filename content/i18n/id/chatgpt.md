# chatgpt

Skrip syel untuk memakai OpenAI ChatGPT dan DALL·E dalam terminal.

## Usage
- `chatgpt` : jalankan dalam mode percakapan interaktif
- `chatgpt -p "Bagaimana cara membuat kriteria ekspresi reguler untuk mencocokkan format alamat surel/email?"` : berikan prompt (pertanyaan) untuk dijawab oleh sang model
- `chatgpt -m gpt-4` : jalankan dalam mode interaktif menggunakan suatu model (model default adalah `gpt-3.5-turbo`)
- `chatgpt -i "Anda adalah Rick, dari serial Rick and Morty. Tanggapi pertanyaan dengan gayanya dan menyertakan lelucon yang menghina."` : jalankan dalam mode interaktif dengan initial prompt, perintah/permintaan awal yang dapat mendefinisikan jenis jawaban yang diharapkan dari sang model
- `echo "Bagaimana cara melihat proses yang berjalan di Ubuntu?" | chatgpt` : alihkan luaran dari program baris perintah lainnya sebagai pertanyaan masukan (prompt) kepada `chatgpt`
- `chatgpt -p "image: Seekor kucing putih"` : buat gambar baru menggunakan model DALL·E

## Links
- More information: https://github.com/0xacx/chatGPT-shell-cli
