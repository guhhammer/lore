# lspci

لیست کردن تمام دستگاههای PCI.

## Usage
- `lspci` : نمایش یک لیست مختصر از دستگاهها
- `lspci -v` : نمایش اطلاعات با جزئیات کامل (verbose) (نکته: فلگ -v برای جزئیات بیشتر قابل تکرار است)
- `lspci -k` : نمایش درایورها و ماژولهای کرنل (kernel) مربوط به هر دستگاه
- `lspci -s 00:18.3` : (select) انتخاب و نمایش اطلاعات یک دستگاه خاص
- `lspci -vm` : نمایش خروجی در فرمت قابل خواندن برای ماشین (machine)
- `lspci -nn` : نمایش کدهای فروشنده (vendor) و دستگاه (device) هم به صورت عددی (numbers) و هم نام (names)

## Links
- More information: https://manned.org/lspci
