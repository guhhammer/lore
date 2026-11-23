# stat

عرض معلومات عن الملف ونظام الملفات.

## Usage
- `stat path/to/file` : عرض خصائص ملف معين مثل الحجم، الأذونات، تواريخ الإنشاء والوصول، وغيرها
- `stat -t path/to/file` : عرض خصائص ملف معين مثل الحجم، الأذونات، تواريخ الإنشاء والوصول، وغيرها بدون تسميات
- `stat -f path/to/file` : عرض معلومات عن نظام الملفات حيث يوجد ملف معين
- `stat -c "%a %n" path/to/file` : عرض أذونات الملف بصيغة ثُمانية فقط
- `stat -c "%U %G" path/to/file` : عرض مالك الملف والمجموعة التابعة له
- `stat -c "%s %n" path/to/file` : عرض حجم ملف معين بالبايت

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/stat-invocation.html
