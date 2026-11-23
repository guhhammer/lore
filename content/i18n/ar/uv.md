# uv

مدير حزم ومشاريع سريع للغة بايثون. بعض الأوامر الفرعية مثل `tool` و `python` لديها توثيق خاص بها.

## Usage
- `uv init` : إنشاء مشروع بايثون جديد في المجلد الحالي
- `uv init path/to/directory` : إنشاء مشروع بايثون جديد في المسار المحدد
- `uv add package` : إضافة تبعية جديدة إلى المشروع
- `uv remove package` : إزالة تبعية من المشروع
- `uv run path/to/script.py` : تشغيل سكربت داخل بيئة المشروع
- `uv run command` : تشغيل أمر داخل بيئة المشروع
- `uv sync` : تحديث بيئة المشروع من `pyproject.toml`
- `uv lock` : إنشاء ملف تأمين (lock file) لتبعيات المشروع

## Links
- More information: https://docs.astral.sh/uv/reference/cli/
