# dockerd

هي عملية مستمرة تعمل في الخلفية تبدأها لتتحكم في حاويات الدوكر.

## Usage
- `dockerd` : قم بتشغيل دوكر في الخلفية
- `dockerd -H unix://path/to/tmp.sock -H tcp://ip_address` : قم بتشغيل دوكر في الخلفية واجعله يستمع علي منفذ معين (يونكس وبروتوكول ضبط الإرسال)
- `dockerd -p path/to/pid_file` : قم بتشغيل دوكر في الخلفية برقم عملية معين
- `dockerd -D` : قم بتشغيل دوكر في وضع التصحيح واكتشاف الأخطاء
- `dockerd -l debug|info|warn|error|fatal` : قم بتشغيل دوكر وحدد له مستوي سجل معين

## Links
- More information: https://docs.docker.com/reference/cli/dockerd/
