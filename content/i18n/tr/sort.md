# sort

Metin dosyalarının satırlarını sırala.

## Usage
- `sort dosya/yolu` : bir dosyayı artan sırada sırala
- `sort -r dosya/yolu` : bir dosyayı azalan sırada sırala
- `sort -f dosya/yolu` : bir dosyayı büyük/küçük harfe duyarsız şekilde sırala
- `sort -n dosya/yolu` : bir dosyayı alfabetik yerine sayısal sıraya göre sırala
- `sort -t : -k 3n /etc/passwd` : alan ayırıcı olarak `:` kullanarak `/etc/passwd` dosyasını satırların 3. alanından itibaren sayısal olarak sırala
- `sort -t : -k 3,3n -k 4,4g /etc/passwd` : sayısal olarak 3. alana göre `/etc/passwd` dosyasını sırala, 3. alandaki değerler eşitse 4. alana göre üslü sayıları dikkate alarak sırala
- `sort -u dosya/yolu` : bir dosyayı yalnızca benzersiz satırları koruyarak sırala
- `sort -o çıktı_dosyası/yolu girdi_dosyası/yolu` : bir dosyayı sırala ve çıktıyı belirtilen çıktı dosyasına yaz (bir dosyayı yerinde sıralamak için kullanılabilir)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sort-invocation.html
