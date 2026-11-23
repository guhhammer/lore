# hostnamectl

الحصول على اسم المضيف أو تعيينه لجهاز الكمبيوتر.

## Usage
- `hostnamectl` : الحصول على اسم المضيف لجهاز الكمبيوتر
- `sudo hostnamectl set-hostname "hostname"` : تعيين اسم المضيف لجهاز الكمبيوتر
- `sudo hostnamectl set-hostname --static "hostname.example.com" && sudo hostnamectl set-hostname --pretty "hostname"` : تعيين اسم مضيف مُنَظَّم لجهاز الكمبيوتر
- `sudo hostnamectl set-hostname --pretty ""` : إعادة تعيين اسم المضيف إلى قيمته الافتراضية

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/hostnamectl.html
