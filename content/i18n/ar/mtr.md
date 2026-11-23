# mtr

أداة Matt's Traceroute: تجمع بين traceroute و ping.

## Usage
- `mtr example.com` : تتبع المسار إلى مضيف وإرسال حزم ping مستمرة لجميع النقاط الوسيطة
- `mtr -n example.com` : تعطيل تعيين عناوين IP وأسماء المضيفين
- `mtr -w example.com` : عرض المخرجات بعد إرسال 10 حزم ping لكل نقطة
- `mtr -4 example.com` : فرض استخدام IPv4 أو IPv6
- `mtr -i 10 example.com` : الانتظار لوقت محدد (بالثواني) قبل إرسال حزمة أخرى إلى نفس النقطة
- `mtr -z example.com` : عرض رقم النظام المستقل (ASN) لكل نقطة
- `mtr -b example.com` : عرض كل من عنوان IP والاسم العكسي لـ DNS

## Links
- More information: https://manned.org/mtr
