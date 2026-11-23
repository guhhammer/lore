# am

مدیر فعالیت های اندروید.

## Usage
- `am start -n com.android.settings/.Settings` : یک فعالیت خاص رو شروع کن
- `am start -a android.intent.action.VIEW -d tel:123` : یک فعالیت خاص رو شروع کن و داده به آن ارسال کن
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : فعالیتی که با دسته بندی و عمل خاصی مطابقت داره رو شروع کن
- `am to-uri -a android.intent.action.VIEW -d tel:123` : یک اینتنت رو به یک URI تبدیل میکنه

## Links
- More information: https://developer.android.com/tools/adb#am
