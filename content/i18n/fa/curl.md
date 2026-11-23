# curl

انتقال داده از/به سرور. از اکثر پروتکلها از جمله HTTP، FTP و POP3 پشتیبانی میکند.

## Usage
- `curl http://example.com -o path/to/file` : دانلود محتوای یک URL و ذخیره آن در یک فایل(با نام دلخواه)
- `curl -O http://example.com/filename` : دانلود یک فایل و ذخیره آن با نام فایل مشخص شده توسط URL
- `curl -f -O -L -C - http://example.com/filename` : دانلود یک فایل، با دنبال کردن تغییرمسیرهای لینک(location redirects) و ادامه خودکار(از سرگیری) انتقال فایل قبلی. درصورت بروز دادن خطای سرور، خطا نمایش داده خواهد شد
- `curl -d 'name=bob' http://example.com/form` : ارسال داده(فرم) رمزگذاری شده (درخواست POST از نوع application/x-www-form-urlencoded). برای خواندن از `stdin`، از --data @file_name یا --data @'-' استفاده کنید
- `curl -H 'X-My-Header: 123' -X PUT http://example.com` : ارسال یک درخواست با استفاده از متود HTTP دلخواه و هدرهای(header) اضافی
- `curl -d '{"name":"bob"}' -H 'Content-Type: application/json' http://example.com/users/1234` : ارسال داده به صورت JSON، با مشخص کردن content-type مناسب
- `curl -u username http://example.com` : مشخص کردن یک نام کاربری و درخواست وارد کردن رمزعبور از سرور، به منظور احراز هویت
- `curl -E client.pem --key key.pem -k https://example.com` : عبور از گواهی و کلید کاربر یک منبع(رد شدن از اعتبارسنجی گواهی)

## Links
- More information: https://curl.se/docs/manpage.html
