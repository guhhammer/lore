# cat

طباعة وسلسلة الملفات.

## Usage
- `cat path/to/file` : طباعة محتوي ملف إلى `stdout`
- `cat path/to/file1 path/to/file2 ... >path/to/output_file` : دمج عدة ملفات في ملف إخراج
- `cat path/to/file1 path/to/file2 ... >>path/to/output_file` : إلحاق عدة ملفات بملف إخراج
- `cat -u /dev/tty12 >/dev/tty13` : نسخ محتويات ملف إلى ملف إخراج دون استخدام الذاكرة المؤقتة
- `cat - >path/to/file` : كتابة `stdin` إلى ملف

## Links
- More information: https://manned.org/cat.1posix
