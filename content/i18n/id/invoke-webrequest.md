# invoke-webrequest

Buat panggilan dan permintaan HTTP/HTTPS. Catatan: Perintah ini hanya dapat digunakan dalam PowerShell.

## Usage
- `Invoke-WebRequest http://example.com -OutFile jalan\menuju\berkas` : unduh konten URL ke file
- `Invoke-WebRequest http://example.com -UseBasicParsing` : hanya tampilkan konten HTML mentah tanpa menguraikan isinya melalui Internet Explorer (khusus PowerShell 3.0-5.1)
- `Invoke-WebRequest -Method Post -Body @{ name='bob' } http://example.com/form` : kirim data form yang telah di encode (permintaan POST atau tipe data `application/x-www-form-urlencoded`)
- `Invoke-WebRequest -Headers @{ X-My-Header = '123' } -Method PUT http://example.com` : kirim sebuah permintaan dengan header tambahan, menggunakan metode HTTP kustom
- `Invoke-WebRequest -Body '{"name":"bob"}' -ContentType 'application/json' http://example.com/users/1234` : kirim data dalam format JSON, Menentukan jenis konten yang sesuai header
- `Invoke-WebRequest -Headers @{ Authorization = "Basic "+ [System.Convert]::ToBase64String([System.Text.Encoding]::ASCII.GetBytes("myusername:mypassword")) } http://example.com` : berikan nama pengguna dan kata sandi untuk otentikasi server
