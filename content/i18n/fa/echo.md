# echo

چاپ ورودی داده شده.

## Usage
- `echo "سلام دنیا"` : چاپ پیام ورودی. نکته : استفاده از علامت نقل قول انتخابی است
- `echo "مسیر سیستم من $PATH است."` : چاپ یک پیام حاوی متغییرهای سیستمی
- `echo -n "سلام دنیا"` : چاپ یک پیام بدون انتقال به خط جدید
- `echo "سلام دنیا" >>file.txt` : افزودن یک پیام به انتهای یک فایل
- `echo -e "Column 1\tColumn 2"` : فعال کردن تفسیر کاراکترهای خاص، برای مثال tab
- `echo $?` : چاپ وضعیت خروج آخرین دستور اجرا شده (نکته: معادل های این دستور در ویندور به ترتیب echo %errorlevel% و $lastexitcode هستند.)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
