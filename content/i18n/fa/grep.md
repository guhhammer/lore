# grep

یافتن الگو در فایل ها به کمک عبارات با قاعده.

## Usage
- `grep "search_pattern" path/to/file` : جستجو یک الگو در یک فایل
- `grep -F "exact_string" path/to/file` : جستجو یک عبارت خاص (معادل مقایسه رشته ای)
- `grep -rnI "search_pattern" path/to/directory` : جستجو بازگشتی یک الگو در تمامی فایل های یک پوشه، نمایش تمامی خطوط منطبق، فایل های باینری را رد میکند
- `grep -Ei "search_pattern" path/to/file` : استفاده از عبارات با قاعده توسعه یافته (با پشتیبانی از `?`، `+`، `{}`، `()`، و `|`)، در حالت حساس به بزرگی کوچکی کاراکتر ها
- `grep --context|--before-context|--after-context 3 "search_pattern" path/to/file` : چاپ 3 خط از قبل و بعد محل انطباق
- `grep -Hn --color=always "search_pattern" path/to/file` : چاپ نام فایل و شماره خط برای هر انطباق با رنگبندی
- `grep -o "search_pattern" path/to/file` : جستجوی خطوط منطبق، چاپ متن منطبق
- `cat path/to/file | grep -v "search_pattern"` : ورودی استاندارد (`stdin`) رو برای الگوهایی که منطبق نیستند جستجو میکند

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
