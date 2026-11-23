# codex

Asisten kode berbasis bahasa alami untuk terminal, ditenagai oleh OpenAI. Membaca dan mengubah berkas-berkas di dalam direktori saat ini untuk memenuhi permintaan.

## Usage
- `codex` : mulai sesi interaktif Codex di dalam direktori saat ini
- `codex "prompt"` : jalankan satu perintah Codex menggunakan sebuah instruksi (prompt)
- `codex --sandbox workspace-write "prompt"` : jalankan perintah dengan mengizinkan Codex mengubah berkas-berkas di dalam ruang kerja (workspace)
- `codex -m nama_model "prompt"` : gunakan model spesifik
- `codex --oss --local-provider lmstudio|ollama "prompt"` : gunakan penyedia model sumber terbuka (open-source) lokal
- `/status` : [Interaktif] Tampilkan konfigurasi sesi dan penggunaan token
- `codex -h` : tampilkan bantuan

## Links
- More information: https://developers.openai.com/codex/cli/reference
