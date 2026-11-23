# touch

ایجاد فایلها و تنظیم زمانهای دسترسی/تغییر.

## Usage
- `touch path/to/file1 path/to/file2 ...` : ایجاد فایلهای مشخص
- `touch -c -a|m path/to/file1 path/to/file2 ...` : تنظیم زمان aدسترسی یا mتغییر فایل به زمان فعلی و عدم cایجاد فایل در صورت عدم وجود
- `touch -c -t YYYYMMDDHHMM.SS path/to/file1 path/to/file2 ...` : تنظیم tزمان فایل به یک مقدار مشخص و عدم cایجاد فایل در صورت عدم وجود
- `touch -c -r path/to/reference_file path/to/file1 path/to/file2 ...` : تنظیم زمان فایلها به زمان فایل rمرجع، و عدم cایجاد فایل در صورت عدم وجود
