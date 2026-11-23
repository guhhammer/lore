# logcat

تخلیه یک لاگ از پیاهم های سیستمی، شامل پشته رهگیری زمان وقوع خطا، و اطلاعات پیام های لاگ شده توسط برنامه ها.

## Usage
- `logcat` : نمایش لاگ سیستمی
- `logcat -f path/to/file` : نوشتن لاگ سیستمی به یک فایل
- `logcat --regex regular_expression` : نمایش خطاهای منطبق با الگوی ورودی
- `logcat --pid pid` : نمایش لاگ های مربوط به یک PID مشخص
- `logcat --pid $(pidof -s package)` : نمایش لاگ های پروسه های مربوط به یک بسته مشخص

## Links
- More information: https://developer.android.com/tools/logcat
