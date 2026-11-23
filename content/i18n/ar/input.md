# input

ارسال (event codes) او (touchscreen gestures) الى اجهزة اندرويد. يمكنك استخدام هذا الامر من خلال `adb shell`.

## Usage
- `input keyevent event_code` : ارسال (event code) او حرف واحد لأجهزة الاندرويد
- `input text "text"` : ارسال نص لجهاز اندرويد (`%s` تمثل مسافة (space))
- `input tap x_position y_position` : ارسال نقرة واحدة لجهاز اندرويد
- `input swipe x_start y_start x_end y_end duration_in_ms` : ارسال سحبة لجهاز انرويد
- `input swipe x_position y_position x_position y_position duration_in_ms` : ارسال سحبة طويلة لجهاز اندرويد من خلال سحبة
