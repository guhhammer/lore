# fzf

أداة بحث تقريبي (fuzzy) لسطر الأوامر.

## Usage
- `find path/to/directory -type f | fzf` : تشغيل `fzf` على جميع الملفات داخل المجلد المحدد
- `ps aux | fzf` : تشغيل `fzf` للعمليات الجارية
- `find path/to/directory -type f | fzf -m >path/to/file` : تحديد ملفات متعددة باستخدام `<Shift Tab>` وكتابتها إلى ملف
- `fzf -q "query"` : تشغيل `fzf` مع نص بحثي محدد
- `fzf -q "^core go$ | rb$ | py$"` : تشغيل `fzf` على إدخالات تبدأ بـ `core` وتنتهي بـ `go` أو `rb` أو `py`
- `fzf -q '!pyc travis'` : تشغيل `fzf` على إدخالات لا تطابق `pyc` وتحتوي على `travis`

## Links
- More information: https://github.com/junegunn/fzf#usage
