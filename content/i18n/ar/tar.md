# tar

أداة أرشفة. غالبًا ما تُستخدم مع طريقة ضغط، مثل `gzip` أو `bzip2`.

## Usage
- `tar cf path/to/target.tar path/to/file1 path/to/file2 ...` : إنشاء أرشيف وكتابته إلى ملف
- `tar czf path/to/target.tar.gz path/to/file1 path/to/file2 ...` : إنشاء أرشيف وكتابته إلى ملف (gzipped)
- `tar czf path/to/target.tar.gz -C path/to/directory .` : إنشاء أرشيف مضغوط من مجلد باستخدام المسارات النسبية
- `tar xvf path/to/source.tar.ext` : فك ضغط ملف أرشيف مضغوط في المجلد الحالي
- `tar xf path/to/source.tar.ext -C path/to/directory` : فك ضغط ملف أرشيف مضغوط في مجلد محدد
- `tar caf path/to/target.tar.xz path/to/file1 path/to/file2 ...` : إنشاء أرشيف مضغوط وكتابته إلى ملف، مع تحديد خوارزمية الضغط تلقائيًا بناءً على امتداد الملف
- `tar tvf path/to/source.tar` : عرض محتويات ملف بالتفصيل
- `tar xf path/to/source.tar --wildcards "*.html"` : فك ضغط الملفات المطابقة لنمط معين من ملف أرشيف

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html
