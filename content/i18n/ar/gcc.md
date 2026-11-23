# gcc

معالجة مسبقة وتجميع ملفات مصدر C و C++، ثم تجميعها وربطها معًا. جزء من GCC (مجموعة مترجمات جنو).

## Usage
- `gcc path/to/source1.c path/to/source2.c ... -o path/to/output_executable` : ترجمة ملفات مصدر متعددة إلى ملف قابل للتنفيذ
- `gcc path/to/source.c -Wall -o output_executable` : تفعيل عرض جميع الأخطاء والتحذيرات
- `gcc path/to/source.c -Wall -g -Og -o path/to/output_executable` : عرض التحذيرات الشائعة، وإضافة رموز التصحيح إلى الإخراج، وتحسين الأداء دون التأثير على التصحيح
- `gcc path/to/source.c -o path/to/output_executable -Ipath/to/header -Lpath/to/library -llibrary_name` : تضمين مكتبات من مسار مختلف
- `gcc -S path/to/source.c` : ترجمة الكود المصدري إلى تعليمات لغة التجميع
- `gcc -c path/to/source.c` : ترجمة الكود المصدري إلى ملف كائن دون ربط
- `gcc path/to/source.c -O1|2|3|fast -o path/to/output_executable` : تحسين البرنامج المترجم لزيادة الأداء
- `gcc --version` : عرض الإصدار

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
