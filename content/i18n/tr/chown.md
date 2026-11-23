# chown

Dosyaların ve dizinlerin kullanıcı ve grup sahipliğini değiştir.

## Usage
- `sudo chown kullanıcı dosya_veya_dizin/yolu` : bir dosyanın/dizinin sahibi olan kullanıcıyı değiştir
- `sudo chown kullanıcı:grup dosya_veya_dizin/yolu` : bir dosyanın/dizinin sahibi olan kullanıcıyı ve grubu değiştir
- `sudo chown kullanıcı: dosya_veya_dizin/yolu` : sahip kullanıcıyı ve grubu, ikisi de `kullanıcı` adını taşıyacak şekilde değiştir
- `chown :grup dosya_veya_dizin/yolu` : bir dosyanın grubunu, mevcut kullanıcının üyesi olduğu bir grupla değiştir
- `sudo chown -R kullanıcı dizin/yolu` : bir dizinin ve içeriğinin sahibini özyinelemeli olarak değiştir
- `sudo chown -h kullanıcı sembolik_bağlantı/yolu` : bir sembolik bağlantının sahibini değiştir
- `sudo chown --reference referans_dosyası/yolu dosya_veya_dizin/yolu` : bir dosyanın/dizinin sahibini bir referans dosyasıyla aynı olacak şekilde değiştir
