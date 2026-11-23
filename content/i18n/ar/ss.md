# ss

أداة للتحقيق في المقابس (Sockets) و المنافذ.

## Usage
- `ss -a --tcp|--udp|--raw|--unix` : عرض جميع مقابس TCP/UDP/RAW/UNIX
- `ss state|exclude bucket|big|connected|synchronized|...` : تصفية مقابس TCP حسب الحالات، تضمين/استبعاد
- `ss -t src :443` : عرض جميع مقابس TCP المتصلة بمنفذ HTTPS المحلي (443)
- `ss -lt src :8080` : عرض جميع مقابس TCP التي تستمع على المنفذ المحلي 8080
- `ss -pt dst :ssh` : عرض جميع مقابس TCP مع العمليات المتصلة بمنفذ SSH الخارجي
- `ss -u 'sport == :source_port and dport == :destination_port'` : عرض جميع مقابس UDP المتصلة بمنفذ مصدر ومنفذ وجهة محددين
- `ss -4t src 192.168/16` : عرض جميع مقابس TCP IPv4 المتصلة محليًا على الشبكة الفرعية 192.168.0.0/16
- `ss -K dst 192.168.1.17 dport = port` : إنهاء اتصال مقبس IPv4 أو IPv6 مع عنوان الوجهة 192.168.1.17 والمنفذ 8080

## Links
- More information: https://manned.org/ss
