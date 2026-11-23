# tldr

يعرض صفحات مساعدة مبسطة للأوامر في سطر الأوامر، مستمدة من مشروع tldr-pages. ملاحظة: الخيارات `--language` و `--list` ليست مطلوبة وفقًا للمواصفات، ولكن معظم العملاء يدعمونها.

## Usage
- `tldr command` : عرض صفحة tldr لأمر معين (تلميح: هذا ما أوصلك إلى هنا!)
- `tldr command subcommand` : عرض صفحة tldr لأمر فرعي معين
- `tldr -L language_code command` : عرض صفحة tldr لأمر بلغة معينة (إن وجدت، وإلا سيتم الرجوع إلى الإنجليزية)
- `tldr -p android|common|freebsd|linux|osx|netbsd|openbsd|sunos|windows command` : عرض صفحة tldr لأمر من نظام تشغيل معين
- `tldr -u` : تحديث ذاكرة التخزين المؤقت لصفحات tldr
- `tldr -l` : عرض قائمة بجميع الصفحات المتاحة للمنصة الحالية وللأوامر الشائعة
- `tldr -l | grep command | column` : عرض جميع الصفحات الفرعية المتاحة لأمر معين
- `tldr -l | shuf -n 1 | xargs tldr` : عرض صفحة tldr لأمر عشوائي
