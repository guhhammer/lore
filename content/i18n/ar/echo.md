# echo

طباعة نص أو قيمة متغير في الـ terminal.

## Usage
- `echo "Hello World"` : طباعة نص على الشاشة. ملاحظة: علامات الاقتباس اختيارية
- `echo "My path is $PATH"` : طباعة نص يتضمن قيمة متغير بيئة
- `echo -n "Hello World"` : طباعة نص دون إضافة سطر جديد في النهاية
- `echo "Hello World" >>file.txt` : إضافة نص إلى نهاية ملف
- `echo -e "Column 1\tColumn 2"` : تفعيل تفسير الرموز الخاصة مثل `\t` للجدولة و`\n` للسطر الجديد
- `echo $?` : عرض رمز الخروج للأمر السابق (في Windows استخدم `echo %errorlevel%`، وفي PowerShell `$lastexitcode`)
- `echo "Hello World" | program` : توجيه نص إلى مدخل برنامج آخر

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
