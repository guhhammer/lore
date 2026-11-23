# tcpdump

Создавать дамп сетевого трафика.

## Usage
- `tcpdump -D` : вывести список доступных сетевых интерфейсов
- `sudo tcpdump -i eth0` : захватить трафик с определённого интерфейса
- `sudo tcpdump -A tcp` : захватить весь TCP-трафик, показывая содержимое в консоли в формате ASCII
- `sudo tcpdump host www.example.com` : захватить трафик, идущий от хоста или к хосту
- `sudo tcpdump -i eth0 src 192.168.1.1 and dst 192.168.1.2 and dst port 80` : захватить трафик с определённого интерфейса, источника, назначения и порта назначения
- `sudo tcpdump net 192.168.1.0/24` : захватить трафик сети
- `sudo tcpdump -w путь/к/файлу_дампа.pcap port not 22` : захватить весь трафик, кроме трафика на порту 22, и записать (write) его в файл дампа
- `tcpdump -r путь/к/файлу_дампа.pcap` : прочитать (read) из указанного файла дампа

## Links
- More information: https://www.tcpdump.org/manpages/tcpdump.1.html
