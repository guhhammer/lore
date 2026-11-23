# wc

عدّ الأسطر والكلمات والبايتات.

## Usage
- `wc -l path/to/file` : عدّ جميع الأسطر في ملف
- `wc -w path/to/file` : عدّ جميع الكلمات في ملف
- `wc -c path/to/file` : عدّ جميع البايتات في ملف
- `wc -m path/to/file` : عدّ جميع الأحرف في ملف (مع أخذ الأحرف متعددة البايتات في الاعتبار مثل الحروف العربية)
- `find . | wc` : عدّ جميع الأسطر والكلمات والبايتات من `stdin`
- `wc -L path/to/file` : حساب طول أطول سطر بعدد الأحرف

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html
