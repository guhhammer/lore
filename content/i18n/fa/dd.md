# dd

تبدیل و کپی یک فایل.

## Usage
- `dd if=path/to/file.iso of=/dev/usb_drive` : یک حافظه قابل حمل با قابلیت بوت شدن میسازد، برای مثال `archlinux-xxx.iso`
- `dd bs=4194304 conv=noerror if=/dev/source_drive of=/dev/dest_drive` : محتویات یک درایو را در مکانی دیگر با بلوک های 4 مگابایتی کپی و همچنین از خطاها صرف نظر میکند
- `dd if=/dev/urandom of=path/to/random_file bs=100 count=1` : یک فایل ۱۰۰ بایتی تصادفی با استفاده از درایور تصادفی هسته بسازید
- `dd if=/dev/zero of=path/to/file_1GB bs=1024 count=1000000` : عملکرد نوشتن دیسک را بسنجید
- `dd if=/dev/drive_device of=path/to/file.img` : یک پشتیبان از سیستم را در یک فایل IMG میسازد
- `dd if=path/to/file.img of=/dev/drive_device` : یک درایو را از یک فایل IMG بازیابی کنید

## Links
- More information: https://manned.org/dd.1p
