# diff

مقایسه فایل(ها) و پوشه(ها).

## Usage
- `diff old_file new_file` : مقایسه فایل ها (فهرست تغییرات فایل های قدیمی به جدید)
- `diff -w old_file new_file` : مقایسه فایل ها، با صرف نظر از فاصله های خالی
- `diff -y old_file new_file` : مقایسه فایل ها، با نمایش تفاوت ها در کنار هم
- `diff -u old_file new_file` : مقایسه فایل ها، به نمایش تفاوت ها به صورت یکپارچه (همانند `git diff`)
- `diff -r old_directory new_directory` : مقایسه بازگشتی پوشه ها (نمایش اسامی متفاوت فایل ها و پوشه ها و همچنین تغییرات فایل ها)
- `diff -r -q old_directory new_directory` : نمایش نام فایل های متفاوت مقایسه شده
- `diff -a -u -N old_file new_file >diff.patch` : از تفاوت دو فایل متنی یک بروزرسانی میسازد، فایل های ناموجود را خالی فرض میکند

## Links
- More information: https://manned.org/diff
