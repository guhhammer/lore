# zip

بستهبندی و فشردهسازی (آرشیو) فایلها در یک آرشیو Zip.

## Usage
- `zip -r path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : افزودن فایلها/پوشهها به یک آرشیو مشخص (به صورت بازگشتی)
- `zip -d path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : حذف فایلها/پوشهها از یک آرشیو مشخص
- `zip -r path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ... -x path/to/excluded_files_or_directories` : آرشیو فایلها/پوشهها با مستثنی کردن موارد مشخص شده
- `zip -r -0..9 path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : آرشیو فایلها/پوشهها با سطح فشردهسازی مشخص (`0` - کمترین، `9` - بیشترین)
- `zip -re path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : ایجاد یک آرشیو رمزگذاری شده با رمز عبور مشخص
- `zip -rs 3g path/to/compressed.zip path/to/file_or_directory1 path/to/file_or_directory2 ...` : آرشیو فایلها/پوشهها در یک آرشیو چند بخشی تقسیم شده (مثلاً بخشهای ۳ گیگابایتی)
- `zip -sf path/to/compressed.zip` : نمایش محتویات یک آرشیو مشخص

## Links
- More information: https://manned.org/zip
