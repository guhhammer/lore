# tar

ابزار آرشیو کردن. اغلب با یک روش فشردهسازی مانند `gzip` یا `bzip2` ترکیب میشود.

## Usage
- `tar cf path/to/target.tar path/to/file1 path/to/file2 ...` : cایجاد یک آرشیو و نوشتن آن در یک fفایل
- `tar czf path/to/target.tar.gz path/to/file1 path/to/file2 ...` : cایجاد یک آرشیو gzip شده و نوشتن آن در یک fفایل
- `tar czf path/to/target.tar.gz -C path/to/directory .` : cایجاد یک آرشیو gzip شده (فشرده) از یک پوشه با استفاده از مسیرهای نسبی
- `tar xvf path/to/source.tar.ext` : استخراج x یک آرشیو (فشرده) fفایل در پوشه فعلی به صورت vکامل
- `tar xf path/to/source.tar.ext -C path/to/directory` : استخراج x یک آرشیو (فشرده) fفایل در پوشه مقصد
- `tar caf path/to/target.tar.xz path/to/file1 path/to/file2 ...` : cایجاد یک آرشیو فشرده و نوشتن آن در یک fفایل، با استفاده از پسوند فایل برای تعیین aخودکار برنامه فشردهسازی
- `tar tvf path/to/source.tar` : نمایش t محتویات یک فایل ftar به صورت vکامل
- `tar xf path/to/source.tar --wildcards "*.html"` : استخراج x فایلهای منطبق با یک الگو از یک آرشیو fفایل

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html
