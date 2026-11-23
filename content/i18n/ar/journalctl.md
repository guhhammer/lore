# journalctl

استعلم عن سجلات نظام التشغيل (systemd journal).

## Usage
- `journalctl -n عدد_الأسطر -f` : [-n] اعرض السجلات الأحدث بعدد معين وتابع الرسائل الجديدة فور حدوثها
- `journalctl -b -1 -p 3` : [-b] اعرض جميع الرسائل ذات مستوى الأولوية 3 (الأخطاء) الخاصة بجلسة التشغيل السابقة
- `journalctl -u اسم_الوحدة` : [-u] اعرض جميع الرسائل الخاصة بوحدة (unit) معينة في النظام
- `journalctl _SYSTEMD_INVOCATION_ID=$(systemctl show --value --property=InvocationID اسم_الوحدة)` : [_] اعرض السجلات لوحدة معينة منذ آخر مرة تم تشغيلها فيها
- `journalctl -S now|today|yesterday|tomorrow|... -U "السنة-الشهر-اليوم الساعات:الدقائق:الثواني"` : [-S] تصفح الرسائل وضفّها ضمن نطاق زمني محدد (باستخدام طابع زمني أو كلمات دلالية)
- `journalctl _PID=معرف_العملية` : [_] اعرض جميع الرسائل الخاصة بمعرّف عملية (PID) محددة
- `journalctl مسار/الملف_التنفيذي` : [/] اعرض جميع الرسائل الخاصة بملف تنفيذي معين
- `journalctl --vacuum-time 2d` : [--] احذف سجلات النظام القديمة التي يتجاوز عمرها يومين

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html
