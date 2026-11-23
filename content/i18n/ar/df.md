# df

عرض نظرة عامة على استخدام مساحة القرص لنظام الملفات.

## Usage
- `df` : عرض جميع أنظمة الملفات واستخدامها للقرص
- `df -h` : عرض جميع أنظمة الملفات واستخدامها للقرص بصيغة قابلة للقراءة البشرية
- `df path/to/file_or_directory` : عرض نظام الملفات واستخدامه للقرص الذي يحتوي على ملف أو مُجَلَّد معين
- `df -i` : تضمين إحصائيات حول عدد العقد المتاحة (inodes)
- `df -x squashfs -x tmpfs` : عرض أنظمة الملفات مع استثناء أنواع محددة
- `df -T` : عرض أنواع أنظمة الملفات

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/df-invocation.html
