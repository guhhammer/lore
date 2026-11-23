# gpg

برنامج GNU Privacy Guard.

## Usage
- `gpg --full-gen-key` : إنشاء مفتاح GPG عام وخاص بطريقة تفاعلية
- `gpg --clearsign doc.txt` : توقيع الملف `doc.txt` دون تشفير (يتم حفظ الإخراج في ملف `doc.txt.asc`)
- `gpg -es -r alice@example.com -r bob@example.com doc.txt` : تشفير وتوقيع الملف `doc.txt` للمستخدمين alice@example.com و bob@example.com (يتم حفظ الإخراج في ملف `doc.txt.gpg`)
- `gpg -c doc.txt` : تشفير `doc.txt` باستخدام كلمة مرور فقط (يتم حفظ الإخراج في ملف `doc.txt.gpg`)
- `gpg -d doc.txt.gpg` : فك تشفير `doc.txt.gpg` (يتم عرض الإخراج على `stdout`)
- `gpg --import public.gpg` : استيراد مفتاح عام
- `gpg --export -a alice@example.com` : تصدير المفتاح العام للمستخدم alice@example.com (يتم عرض الإخراج على `stdout`)
- `gpg --export-secret-keys -a alice@example.com` : تصدير المفتاح الخاص للمستخدم alice@example.com (يتم عرض الإخراج على `stdout`)

## Links
- More information: https://gnupg.org/documentation/manuals/gnupg/Invoking-GPG.html
