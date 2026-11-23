# aria2c

ابزاری برای دانلود سریع. قابلیت پشتیبانی از Http(s), FTP, SFTP, BitTorrent, Metalink.

## Usage
- `aria2c "url"` : دانلود لینک موردنظر و ذخیره در فایل
- `aria2c -o path/to/file "url"` : دانلود یک فایل از لینک موردنظر با اسم خروجی دلخواه
- `aria2c -Z "url1" "url2" ...` : دانلود چند فایل مختلف به صورت همزمان
- `aria2c "url1" "url2" ...` : دانلود از چند لینک مختلف
- `aria2c --input-file=path/to/file --max-concurrent-downloads=number_of_downloads` : دانلود لینک های لیست شده در یک فایل همراه با تنظیم تعداد دانلود های همزمان
- `aria2c -s number_of_connections "url"` : دانلود با چندین اتصال مختلف
- `aria2c --ftp-user=username --ftp-passwd=password "url"` : دانلود از FTP با نام کاربری و رمزعبور
- `aria2c --max-download-limit=speed "url"` : محدود کردن سرعت دانلود با واحد بایت بر ثانیه

## Links
- More information: https://aria2.github.io/manual/en/html/aria2c.html
