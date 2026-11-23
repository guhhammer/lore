# systemctl

التحكم في مدير نظام systemd والخدمات.

## Usage
- `systemctl status` : عرض جميع الخدمات قيد التشغيل
- `systemctl --failed` : عرض الوحدات الفاشلة
- `systemctl start|stop|restart|reload|status unit` : بدء/إيقاف/إعادة تشغيل/إعادة تحميل/عرض حالة خدمة
- `systemctl enable|disable unit` : تمكين/تعطيل وحدة ليتم تشغيلها عند بدء تشغيل النظام
- `systemctl daemon-reload` : إعادة تحميل systemd والبحث عن وحدات جديدة أو متغيرة
- `systemctl is-active|is-enabled|is-failed unit` : التحقق مما إذا كانت الوحدة نشطة/مُمكّنة/فاشلة
- `systemctl list-units -t service|socket|automount --state failed|running` : عرض جميع وحدات الخدمة/المقبس/التركيب التلقائي مع التصفية حسب الحالة (قيد التشغيل/فاشلة)
- `systemctl cat unit` : عرض محتويات ومسار ملف الوحدة

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
