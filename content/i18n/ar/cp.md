# cp

نسخ الملفات والمجلدات.

## Usage
- `cp path/to/source_file path/to/target_file` : نسخ ملف إلى موقع آخر
- `cp path/to/source_file path/to/target_parent_directory` : نسخ ملف داخل مجلد مع الاحتفاظ باسمه
- `cp -r path/to/source_directory path/to/target_directory` : نسخ مجلد بالكامل بما يشمل المجلدات الفرعية (إن وُجد المجلد الهدف، يُنسخ المصدر بداخله)
- `cp -vr path/to/source_directory path/to/target_directory` : نسخ مجلد بالكامل مع عرض اسم كل ملف أثناء نسخه
- `cp -t path/to/destination_directory path/to/file1 path/to/file2 ...` : نسخ عدة ملفات إلى مجلد في آنٍ واحد
- `cp -i *.ext path/to/target_directory` : نسخ جميع ملفات بامتداد معين مع طلب تأكيد عند الكتابة فوق ملف موجود
- `cp -L link path/to/target_directory` : اتباع الروابط الرمزية قبل النسخ
- `cp --parents source/path/to/file path/to/target_file` : الحفاظ على هيكل المسار الكامل أثناء النسخ، وإنشاء المجلدات الوسيطة عند الحاجة

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
