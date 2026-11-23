# am

مدير الأنشطة في أندرويد.

## Usage
- `am start -n com.android.settings/.Settings` : ابدأ نشاطا بتحديد اسم الحزمة/المكوّن بالكامل
- `am start -a android.intent.action.VIEW -d tel:123` : ابدأ فعلا (action) ومرّر بيانات (data) له
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : ابدأ نشاطا يطابق فعلا (action) وفئة (category) معيّنة
- `am to-uri -a android.intent.action.VIEW -d tel:123` : حوّل Intent إلى URI

## Links
- More information: https://developer.android.com/tools/adb#am
