# ip route list

أمر فرعي لعرض جدول التوجيه في نظام الشبكة.

## Usage
- `ip r l` : عرض جدول التوجيه الرئيسي
- `ip r l t main|254` : عرض جدول التوجيه الرئيسي (نفس المثال الأول)
- `ip r l t local|255` : عرض جدول التوجيه المحلي
- `ip r l t all|unspec|0` : عرض جميع جداول التوجيه
- `ip r l dev ethX` : عرض المسارات الخاصة بواجهة معينة فقط
- `ip r l s link` : عرض المسارات داخل نطاق معين
- `ip r l c` : عرض ذاكرة التخزين المؤقت للمسارات
- `ip -6|-4 r` : عرض مسارات IPv6 أو IPv4 فقط

## Links
- More information: https://manned.org/ip-route
