# node

یک runtime برای اجرای جاوا اسکریپت. (Node.js).

## Usage
- `node path/to/file` : اجرای یک فایل جاوااسکریپت
- `node` : شروع یک REPL (پوسته تعاملی)
- `node --watch path/to/file` : اجرای فایل مشخصشده و راهاندازی مجدد فرآیند هنگام تغییر در یک فایل وارد شده (نیازمند نسخه Node.js 18.11+)
- `node -e "code"` : ارزیابی کد جاوااسکریپت با ارسال آن بهعنوان یک آرگومان
- `node -p "process.versions"` : ارزیابی و چاپ نتیجه، مفید برای چاپ نسخههای وابستگیهای Node
- `node --no-lazy --inspect-brk path/to/file` : فعال کردن حالت Inspector، توقف اجرای برنامه تا زمانی که دیباگر به کد متصل شود و کد کاملاً تحلیل شود

## Links
- More information: https://nodejs.org/docs/latest/api/cli.html#options
