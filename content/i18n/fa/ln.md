# ln

این دستور برای ایجاد ارتباط(link) به فایل ها و پوشه ها(Directories) استفاده می شود.

## Usage
- `ln -s /path/to/file_or_directory path/to/symlink` : ایجاد یک ارتباط نمادین (symbolic link) به یک فایل یا پوشه
- `ln -sf /path/to/new_file path/to/symlink` : جایگزینی یک ارتباط نمادین موجود، برای اشاره به یک فایل متفاوت
- `ln /path/to/file path/to/hardlink` : ایجاد یک لینک سخت (hard link) به یک فایل

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/ln-invocation.html
