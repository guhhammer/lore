# chown

مالک فایل یا پوشه را تغییر میدهد.

## Usage
- `chown user path/to/file_or_directory` : تغییر مالک یه فایل یا پوشه
- `chown user:group path/to/file_or_directory` : تغییر کاربر و گروه مالک فایل
- `chown -R user path/to/directory` : تغییر بازگشتی مالک یه پوشه و محتویات آن
- `chown -h user path/to/symlink` : تغییر مالک یک فایل میانبر(به فایل دیگری اشاره میکند)
- `chown --reference path/to/reference_file path/to/file_or_directory` : تغییر مالک یک فایل/پوشه برای همسان شدن با فایل مرجع
