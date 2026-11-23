# apt

أداة لإدارة الحزم للتوزيعات القائمة على ديبيان. بديل لـ `apt-get` عند الاستخدام الفعال في إصدارات أوبونتو 16.04 وما بعده.

## Usage
- `sudo apt update` : تحديث قائمة الحزم الموجودة وإصداراتها (يوصى بتشغيله قبل أي أمر `apt` آخر)
- `apt search package` : البحث عن حزمة بالاسم أو الوصف
- `apt list package` : البحث عن الحزم بالاسم فقط (يدعم استخدام wildcards مثل `*`)
- `apt show package` : عرض معلومات تفصيلية حول حزمة معينة
- `sudo apt install package` : تثبيت حزمة جديدة، أو تحديثها إلى آخر إصدار متوفر
- `sudo apt remove package` : إزالة حزمة معينة (استخدم `purge` لحذف ملفات الإعدادات الخاصة بالحزمة)
- `sudo apt upgrade` : تحديث جميع الحزم المثبتة إلى أجدد الإصدارات المتوفرة
- `apt list -i` : إظهار قائمة جميع الحزم المثبتة

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
