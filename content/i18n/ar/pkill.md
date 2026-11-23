# pkill

إرسال إشارة إلى العملية حسب الاسم. يُستخدم غالبًا لإيقاف العمليات.

## Usage
- `pkill "process_name"` : إنهاء جميع العمليات التي تطابق اسم محدد
- `pkill -f "command_name"` : إنهاء جميع العمليات التي تطابق الأمر الكامل بدلًا من مجرد اسم العملية
- `pkill -9 "process_name"` : فرض إنهاء العمليات المطابقة (لا يمكن حظره)
- `pkill -USR1 "process_name"` : إرسال إشارة `SIGUSR1` إلى العمليات التي تطابق الاسم المحدد
- `pkill -o "firefox"` : إنهاء العملية الرئيسية لـ `firefox` لإغلاق المتصفح

## Links
- More information: https://manned.org/pkill
