# go

إدارة شيفرة Go. بعض الأوامر الفرعية مثل `build` لها توثيق استخدام خاص بها.

## Usage
- `go get package_path` : تنزيل وتثبيت حزمة محددة بمسار الاستيراد
- `go run file.go` : ترجمة وتشغيل ملف مصدر (يجب أن يحتوي على الحزمة `main`)
- `go build -o executable file.go` : ترجمة ملف مصدر إلى ملف تنفيذي باسم محدد
- `go build` : ترجمة الحزمة الموجودة في المجلد الحالي
- `go test` : تنفيذ جميع اختبارات الحزمة الحالية (يجب أن تنتهي الملفات بـ `_test.go`)
- `go install` : ترجمة وتثبيت الحزمة الحالية
- `go mod init module_name` : تهيئة وحدة جديدة في المجلد الحالي

## Links
- More information: https://pkg.go.dev/cmd/go
