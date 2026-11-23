# xz

ضغط أو فك ضغط ملفات XZ و LZMA.

## Usage
- `xz path/to/file` : ضغط ملف باستخدام xz
- `xz -d path/to/file.xz` : فك ضغط ملف XZ
- `xz -F lzma path/to/file` : ضغط ملف باستخدام خوارزمية lzma
- `xz -d -F lzma path/to/file.lzma` : فك ضغط ملف LZMA
- `xz -d -c path/to/file.xz` : فك ضغط ملف وكتابته إلى المخرجات القياسية (`stdout`) (يتضمن `--keep`)
- `xz -k path/to/file` : ضغط ملف بدون حذف النسخة الأصلية
- `xz -0 path/to/file` : ضغط ملف باستخدام أسرع ضغط
- `xz -9 path/to/file` : ضغط ملف باستخدام أكفأ ضغط

## Links
- More information: https://manned.org/xz
