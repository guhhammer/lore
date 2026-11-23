# babeld

방화벽-스타일 필터를 사용하는 Babel용 라우팅 데몬입니다.

## Usage
- `babeld -c 경로/대상/ports.conf -c 경로/대상/filters.conf -c 경로/대상/interfaces.conf` : 하나 이상의 구성(configuration) 파일을 사용하여 데몬을 시작 (순서대로 읽음)
- `babeld -D` : 시작 후 데몬화(Deamonize)
- `babeld -C 'redistribute metric 256'` : 구성(Configuration) 명령어를 지정
- `babeld eth0 eth1 wlan0` : 작동할 인터페이스를 지정

## Links
- More information: https://www.irif.fr/~jch/software/babel/babeld.html
