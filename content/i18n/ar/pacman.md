# pacman

أداة مدير الحزم لنظام Arch Linux.

## Usage
- `sudo pacman -Syu` : مزامنة وتحديث جميع الحزم
- `sudo pacman -S package` : تثبيت حزمة جديدة
- `sudo pacman -Rs package` : إزالة حزمة والتبعيات الخاصة بها
- `pacman -Ss "search_pattern"` : البحث في قاعدة بيانات الحزم عن تعبير نمطي (`regex`) أو كلمة مفتاحية
- `pacman -F "file_name"` : البحث في قاعدة البيانات عن الحزم التي تحتوي على ملف محدد
- `pacman -Qe` : عرض الحزم المثبتة بشكل صريح (تم تثبيتها يدويًا بواسطة المستخدم) مع إصداراتها
- `pacman -Qtdq` : عرض الحزم اليتيمة (المثبتة كـ تبعيات ولكنها غير مطلوبة من قبل أي حزمة)
- `sudo pacman -Scc` : تفريغ ذاكرة التخزين المؤقت بالكامل لـ `pacman`

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
