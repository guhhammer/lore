# alias

ایجاد نام مستعار -- کلمه ای که جایگزین یک دستور می باشد. نام های مستعار طول عمری برابر با جلسه جاری شل مربوطه دارند مگر اینکه در فایل های پیکربندی شل مربوط نظیر `~/.bashrc` تعریف شوند.

## Usage
- `alias` : نمایش تمامی نام های مستعار
- `alias word="command"` : ایجاد یک نام مستعار
- `alias word` : نمایش نام مستعار مرتبط با کلمه مشخص شده
- `unalias word` : حذف یک نام مستعار
- `alias rm="rm -i"` : تغییر `rm` به نسخه تعاملی با تعریف نام مستعار
- `alias la="ls -a"` : تعریف `la` به عنوان میانبری برای `ls -a`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
