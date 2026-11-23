# ulimit

الحصول على وتعيين حدود الموارد لعمليات المستخدم.

## Usage
- `ulimit -a` : عرض خصائص جميع حدود المستخدم
- `ulimit -H -n` : عرض الحد الأقصى الثابت لعدد الملفات التي يمكن فتحها في نفس الوقت
- `ulimit -S -n` : عرض الحد القابل للتغيير لعدد الملفات التي يمكن فتحها في نفس الوقت
- `ulimit -u 30` : تعيين الحد الأقصى لعدد العمليات لكل مستخدم

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-ulimit
