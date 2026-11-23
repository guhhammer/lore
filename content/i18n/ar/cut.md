# cut

استخراج حقول من `stdin` أو من الملفات.

## Usage
- `command | cut --characters|fields 1|1,10|1-10|1-|-10` : طباعة نطاق cحرف/fحقل محدد من كل سطر
- `command | cut --delimiter "," --fields 1` : طباعة نطاق fحقل محدد من كل سطر باستخدام dفاصل معين
- `cut --characters 1 path/to/file` : طباعة نطاق cحرف معين من كل سطر في ملف محدد
- `command | cut --zero-terminated --fields 1` : طباعة fحقول محددة من أسطر منتهية بـ `NUL` (مثل الناتج من `find . -print0`) بدلاً من أسطر جديدة

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cut-invocation.html
