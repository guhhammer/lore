# apt-get

أداة إدارة الحزم لديبيان وأوبونتو. ابحث عن الحزم باستخدام `apt-cache`.

## Usage
- `sudo apt-get update` : تحديث قائمة الحزم الموجودة وإصداراتها (يوصى بتشغيله قبل أي أمر `apt-get` آخر)
- `sudo apt-get install package` : تثبيت حزمة معينة، أو تحديثها إلى آخر إصدار متوفر
- `sudo apt-get remove package` : إزالة حزمة معينة
- `sudo apt-get purge package` : إزالة حزمة معينة وملفات الإعدادات الخاصة بها
- `sudo apt-get upgrade` : تطوير جميع الحزم المثبتة إلى أجدد الإصدارات المتوفرة
- `sudo apt-get autoclean` : تنظيف المستودع المحلي - إزالة ملفات الحزم (`.deb`) من التنزيلات المعطلة التي لم يعد من الممكن تنزيلها
- `sudo apt-get autoremove` : إزالة جميع الحزم التي لم تعد مطلوبة
- `sudo apt-get dist-upgrade` : تطوير الحزم المثبتة (مثل `upgrade`)، ولكن تقوم بإزالة الحزم القديمة وتثبيت حزم إضافية لتلبية التوابع الجديدة

## Links
- More information: https://manned.org/apt-get.8
