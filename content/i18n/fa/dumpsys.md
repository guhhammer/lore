# dumpsys

ارائه اطلاعات درباره سیستم سرویس اندروید. این دستور فقط از طریق `adb shell` قابل اجراست.

## Usage
- `dumpsys` : دریافت اطلاعات عیب بای تمامی سرویس های سیستمی
- `dumpsys service` : دریافت اطلاعات عیب یابی یک سرویس سیستمی مشخص
- `dumpsys -l` : فهرستی از تمامی سرویس هایی که `dumpsys` میتواند اطلاعات بدهد
- `dumpsys service -h` : ورودی های یک سرویس مشخص را فهرست میکند
- `dumpsys --skip service` : حذف یک سرویس مشخص از خروجی عیب یابی
- `dumpsys -t 8` : تعیین فرصت زمانی بر مبنای ثانیه (پیش فرض 10ثانیه)

## Links
- More information: https://developer.android.com/tools/dumpsys
