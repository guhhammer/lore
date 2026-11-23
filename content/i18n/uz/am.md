# am

Android faoliyat boshqaruvi.

## Usage
- `am start -n com.android.settings/.Settings` : anniq bir faoliyatni boshlash
- `am start -a android.intent.action.VIEW -d tel:123` : faoliyatni boshlash va unga malumot o'tkazish
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : biron harakat va kategoriyaga mos keluvchi faoliyatni boshlash
- `am to-uri -a android.intent.action.VIEW -d tel:123` : intentni URI ga o'zgartirish

## Links
- More information: https://developer.android.com/tools/adb#am
