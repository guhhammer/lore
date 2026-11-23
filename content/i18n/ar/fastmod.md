# fastmod

أداة للاستبدال الجزئي للنصوص في قاعدة الأكواد لديك. التعبيرات النمطية يعالجها قفص من بضاعة رست وهو `regex`.

## Usage
- `fastmod regex replacement` : استبدال بالتعبيرات النمطية في كل ملفات المسار الحالي وأبنائه في الملفات غير المُتجاهلة بـ `.ignore` أو `.gitignore`
- `fastmod --ignore-case regex replacement -- path/to/file_or_directory1 path/to/file_or_directory2 ...` : استبدال متجاهلا حالة الحرف في ملف أو في ملفات مسار
- `fastmod regex replacement --dir path/to/directory --iglob '**/*.{js,json}'` : استبدال بالتعبيرات النمطية مع تحديد المكان الذي يُستبدل فيه
- `fastmod --fixed-strings exact_string replacement --extensions json,js` : استبدال بالنص مُطابقةً (وليس التعبيرات النمطية)، في ملفات امتداداتهم إما js أو JSON فحسب
- `fastmod --accept-all --fixed-strings exact_string replacement` : استبدال بجميع النصوص مُطابقةً، مباشرة دون مِحَثِّ تأكيد (prompt)
- `fastmod --accept-all --print-changed-files --fixed-strings exact_string replacement` : استبدال بجميع النصوص مُطابقةً، مباشرة دون تأكيد، مع طباعة الملفات المُستبدل فيها

## Links
- More information: https://github.com/facebookincubator/fastmod
