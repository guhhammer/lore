# tail

عرض الجزء الأخير من ملف.

## Usage
- `tail -n count path/to/file` : عرض آخر 'عدد' من الأسطر في ملف
- `tail -n +count path/to/file` : طباعة محتوى ملف بدءًا من سطر معين
- `tail -c count path/to/file` : طباعة عدد معين من البايتات من نهاية ملف معين
- `tail -f path/to/file` : طباعة آخر الأسطر من ملف معين والاستمرار في قراءته حتى الضغط على `<Ctrl c>`
- `tail -F path/to/file` : الاستمرار في قراءة الملف حتى الضغط على `<Ctrl c>`، حتى لو كان غير متاح
- `tail -n count -s seconds -f path/to/file` : عرض آخر 'عدد' من الأسطر في 'ملف' وتحديث العرض كل 'عدد' من الثواني

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html
