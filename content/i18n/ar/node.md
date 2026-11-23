# node

بيئة تشغيل JavaScript للخادم (Node.js).

## Usage
- `node path/to/file` : تشغيل ملف JavaScript
- `node` : بدء REPL (وحدة تحكم تفاعلية)
- `node --watch path/to/file` : تنفيذ الملف المحدد مع إعادة تشغيل العملية عند تغيير ملف مستورد (يتطلب Node.js إصدار 18.11+)
- `node -e "code"` : تنفيذ كود JavaScript من سطر الأوامر
- `node -p "process.versions"` : تنفيذ وطباعة النتيجة، مفيد لطباعة إصدارات تبعيات node
- `node --no-lazy --inspect-brk path/to/file` : تفعيل المصحح (inspector)، مع إيقاف التنفيذ حتى يتم الاتصال بمصحح الأخطاء (debugger) بمجرد تحليل الكود المصدري بالكامل

## Links
- More information: https://nodejs.org/docs/latest/api/cli.html#options
