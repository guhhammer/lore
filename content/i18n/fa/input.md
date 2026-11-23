# input

ارسال کد رویداد یا ورودی صفحه نمایش به یک دستگاه اندروید. این دستور فقط از طریق `adb shell` قابل اجراست.

## Usage
- `input keyevent event_code` : ارسال کد رویداد یک کاراکتر به یک دستگاه اندروید
- `input text "text"` : ارسال یک متن به یک دستگاه اندروید (`%s` نمایانگر فاصله است)
- `input tap x_position y_position` : ارسال یک ضربه به یک دستگاه اندروید
- `input swipe x_start y_start x_end y_end duration_in_ms` : ارسال حرکت افقی عمودی به یک دستگاه اندروید
- `input swipe x_position y_position x_position y_position duration_in_ms` : ارسال یک نگهداشتن بلند مدت به یک دستگاه اندرویدی از طریق حرکت افقی عمودی
