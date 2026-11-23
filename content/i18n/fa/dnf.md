# dnf

ابزار مدیریت بستهها برای RHEL، Fedora و CentOS (جایگزین `yum`).

## Usage
- `sudo dnf upgrade` : ارتقاء بستههای نصب شده به جدیدترین نسخههای موجود
- `dnf search keyword1 keyword2 ...` : جستجوی بستهها بر اساس کلمات کلیدی
- `dnf info package` : نمایش جزئیات یک بسته
- `sudo dnf install package1 package2 ...` : نصب یک بسته جدید (از `-y` برای تأیید اتوماتیک تمام پنجرهها استفاده کنید)
- `sudo dnf remove package1 package2 ...` : حذف یک بسته
- `dnf list --installed` : لیست بستههای نصب شده
- `dnf provides command` : یافتن بستههایی که دستور مشخصی را ارائه میدهند
- `dnf history` : مشاهده تاریخچه تمام عملیاتهای گذشته

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
