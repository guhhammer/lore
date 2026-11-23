# rm

يستخدم الأمر لحذف الملفات او المجلدات.

## Usage
- `rm path/to/file1 path/to/file2 ...` : حذف ملفات محددة
- `rm -f path/to/file1 path/to/file2 ...` : حذف ملفات محددة وتجاهل الملفات الغير موجودة
- `rm -i path/to/file1 path/to/file2 ...` : حذف ملفات محددة مع واجهة تفاعلية قبل اي حذف للتأكد
- `rm -v path/to/file1 path/to/file2 ...` : حذف ملفات محددة مع عرض تفاصيل حول كل عملية حذف
- `rm -r path/to/file_or_directory1 path/to/file_or_directory2 ...` : حذف ملفات ومجلدات محددة بشكل تسلسلي
- `rm -d path/to/directory` : حذف المجلدات الفارغة (هذه الطريقة تعتبر آمنة)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/rm-invocation.html
