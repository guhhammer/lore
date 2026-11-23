# tr

ترجمة الأحرف: تنفيذ عمليات الاستبدال بناءً على أحرف مفردة ومجموعات أحرف.

## Usage
- `tr <path/to/file find_character replace_character` : استبدال جميع تكرارات حرف معين في ملف وطباعة النتيجة
- `echo text | tr find_character replace_character` : استبدال جميع تكرارات حرف معين من ناتج أمر آخر
- `tr <path/to/file 'abcd' 'jkmn'` : تعيين كل حرف من المجموعة الأولى إلى الحرف المقابل في المجموعة الثانية
- `tr <path/to/file -d 'input_characters'` : حذف جميع تكرارات مجموعة الأحرف المحددة من المدخلات
- `tr <path/to/file -s 'input_characters'` : ضغط سلسلة من الأحرف المتطابقة إلى حرف واحد
- `tr <path/to/file "[:lower:]" "[:upper:]"` : تحويل محتويات ملف إلى أحرف كبيرة (Upper-case)
- `tr <path/to/file -cd "[:print:]"` : إزالة الأحرف غير القابلة للطباعة من ملف

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tr-invocation.html
