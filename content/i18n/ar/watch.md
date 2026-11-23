# watch

تنفيذ برنامج بشكل دوري ومراقبة المخرجات في وضع ملء الشاشة.

## Usage
- `watch command` : تشغيل أمر بشكل متكرر وعرض النتيجة
- `watch -n 60 command` : إعادة تشغيل أمر كل 60 ثانية
- `watch -d df` : مراقبة مساحة القرص مع تمييز الاختلافات عند ظهورها
- `watch "command1 | command2 | command3"` : تشغيل سلسلة أوامر (بايبلاين) بشكل متكرر وعرض النتيجة
- `watch -g lsblk` : الخروج من `watch` إذا تغير الإخراج المرئي
- `watch -c ls --color=always` : (Terminal) ترجمة رموز التحكم في الطرفية

## Links
- More information: https://manned.org/watch
