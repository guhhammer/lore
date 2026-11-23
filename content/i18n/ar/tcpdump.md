# tcpdump

عرض وتحليل حركة المرور على الشبكة.

## Usage
- `tcpdump -D` : عرض قائمة بواجهات الشبكة المتوفرة
- `sudo tcpdump -i eth0` : التقاط حركة المرور لواجهة شبكة محددة
- `sudo tcpdump -A tcp` : التقاط جميع حركة مرور TCP مع عرض المحتويات (ASCII) في وحدة التحكم
- `sudo tcpdump host www.example.com` : التقاط حركة المرور من أو إلى مضيف محدد
- `sudo tcpdump -i eth0 src 192.168.1.1 and dst 192.168.1.2 and dst port 80` : التقاط حركة المرور من واجهة معينة مع مصدر، وجهة ومنفذ وجهة محددين
- `sudo tcpdump net 192.168.1.0/24` : التقاط حركة المرور لشبكة محددة
- `sudo tcpdump -w dumpfile.pcap port not 22` : التقاط جميع حركة المرور باستثناء حركة المرور على المنفذ 22 وحفظها في ملف
- `tcpdump -r dumpfile.pcap` : قراءة البيانات من ملف محدد

## Links
- More information: https://www.tcpdump.org/manpages/tcpdump.1.html
