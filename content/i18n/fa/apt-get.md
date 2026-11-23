# apt-get

ابزار مدیریت بستههای دبیان و اوبونتو. جستجو در بستهها با استفاده از `apt-cache`.

## Usage
- `sudo apt-get update` : بهروزرسانی لیست بستهها و نسخههای موجود (توصیه میشود که این دستور را قبل از دیگر دستورات `apt-get` اجرا کنید)
- `sudo apt-get install package` : نصب یک بسته یا به روزرسانی آن به آخرین نسخه موجود
- `sudo apt-get remove package` : حذف یک بسته
- `sudo apt-get purge package` : حذف یک بسته و فایلهای پیکربندی آن
- `sudo apt-get upgrade` : ارتقاء تمامی بستههای نصب شده به آخرین نسخههای موجود
- `sudo apt-get autoclean` : پاکسازی مخزن محلی - حذف فایلهای بسته (`.deb`) از دانلودهای متوقف شده که دیگر قابل دانلود نیستند
- `sudo apt-get autoremove` : حذف تمام بستههایی که دیگر نیازی به آنها نیست
- `sudo apt-get dist-upgrade` : ارتقاء بستههای نصب شده (مانند `upgrade`) اما با حذف بستههای قدیمی و نصب بستههای اضافی برای برآورده کردن وابستگیهای جدید

## Links
- More information: https://manned.org/apt-get.8
