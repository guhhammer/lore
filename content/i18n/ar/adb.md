# adb

جسر تصحيح أندرويد (adb) : أداة للإتصال مع نسخة محاكي أندرويد أو مع أجهزة أندرويد المتصلة. بعض الأوامر الفرعية مثل الأمر `shell` لها توثيقات إستخدام خاصة بها.

## Usage
- `adb start-server` : التحقق مما إذا كانت عملية خادم adb قيد التشغيل وبدء تشغيلها إن لم تكن كذلك
- `adb kill-server` : إنهاء عملية خادم adb
- `adb shell` : بدء صَدَفة عن بُعد في نسخة المحاكي أو الجهاز المستهدف
- `adb install -r path/to/file.apk` : دفع تطبيق أندرويد إلى المحاكي أو الجهاز
- `adb pull path/to/device_file_or_directory path/to/local_destination_directory` : نسخ ملف أو دليل من الجهاز المستهدف
- `adb push path/to/local_file_or_directory path/to/device_destination_directory` : نسخ ملف أو دليل إلى الجهاز المستهدف
- `adb devices` : عرض جميع الأجهزة المتصلة
- `adb -s device_id shell` : تحديد الجهاز الذي سيتم إرسال الأوامر إليه في حال وجود عدة أجهزة

## Links
- More information: https://developer.android.com/tools/adb
