# setcap

تعيين الصلاحيات لملف محدد.

## Usage
- `setcap 'cap_net_raw' path/to/file` : تعيين الصلاحية `cap_net_raw` (لاستخدام مآخذ RAW و PACKET) لملف معين
- `setcap 'cap_dac_read_search,cap_sys_tty_config+ep' path/to/file` : تعيين عدة صلاحيات على ملف (`ep` خلف الصلاحية تعني "مفعلة ومسموح بها")
- `setcap -r path/to/file` : إزالة جميع الصلاحيات من ملف
- `setcap -v 'cap_net_raw' path/to/file` : التحقق من أن الصلاحيات المحددة مرتبطة حاليًا بالملف المحدد
- `setcap -n root_uid 'cap_net_admin' path/to/file` : يمكن استخدام المعامل الاختياري `-n root_uid` لتعيين صلاحيات الملف لاستخدامها فقط في نطاق مستخدم معين مع هذا المعرّف الجذري

## Links
- More information: https://manned.org/setcap
