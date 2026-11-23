# pip

مدير الحزم الخاص بلغة بايثون. بعض الأوامر الفرعية مثل `install` لديها توثيق خاص بها.

## Usage
- `pip install package` : تثبيت حزمة (راجع `pip install` لمزيد من خيارات التثبيت)
- `pip install --user package` : تثبيت حزمة في مجلد المستخدم بدلاً من الموقع الافتراضي للنظام
- `pip install -U package` : تحديث حزمة مثبتة
- `pip uninstall package` : إزالة تثبيت حزمة
- `pip freeze >requirements.txt` : حفظ قائمة الحزم المثبتة في ملف
- `pip show package` : عرض معلومات عن حزمة مثبتة
- `pip install -r requirements.txt` : تثبيت الحزم من ملف متطلبات

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip/
