# pacman

واحد مدیریت پکیج آرچ لینوکس.

## Usage
- `sudo pacman -Syu` : همگام سازی و بروز رسانی تمام پکیج ها
- `sudo pacman -S package` : نصب پکیج جدید
- `sudo pacman -Rs package` : حذف یک پکیج به همراه وابستگی هاش
- `pacman -F "file_name"` : جستجو در دیتابیس برای پکیج هایی که با یک فایل خاص تعارض دارند
- `pacman -Q` : لیست کردن پکیج های نصب شده با نسخه آنها
- `pacman -Qe` : لیست کردن تنها پکیج هایی که مستقیما نصب شده اند به همراه نسخه آنها
- `pacman -Qtdq` : لیست کردن پکیج هایی که به عنوان وابستگی نصب شده اند اما توسط هیچ پکیجی استفاده نمیشوند
- `sudo pacman -Scc` : خالی کردن کل کش `pacman`

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
