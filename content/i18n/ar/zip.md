# zip

أداة لضغط وحزم الملفات في أرشيف Zip.

## Usage
- `zip -r path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : إضافة ملفات/مجلدات إلى أرشيف محدد (rبشكل متكرر)
- `zip -d path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : إزالة ملفات/مجلدات من أرشيف محدد (dحذف)
- `zip -r path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ... -x path/to/excluded_files_or_directories` : أرشفة ملفات/مجلدات مع xاستثناء عناصر معينة
- `zip -r -0..9 path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : أرشفة ملفات/مجلدات مع مستوى ضغط محدد (`0` - الأقل، `9` - الأعلى)
- `zip -re path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : إنشاء أرشيف eمشفر باستخدام كلمة مرور محددة
- `zip -rs 3g path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : أرشفة ملفات/مجلدات في أرشيف Zip مقسمs إلى أجزاء متعددة (مثل أجزاء بحجم 3 جيجابايت)
- `zip -sf path/to/compressed.zip` : طباعة محتويات أرشيف محدد

## Links
- More information: https://manned.org/zip
