# ifdata

네트워크 인터페이스에 대한 정보를 표시.

## Usage
- `ifdata -p eth0` : 지정된 인터페이스의 전체 구성을 표시
- `ifdata -e eth0` : 종료 코드를 통해 지정된 인터페이스의 존재(existence)를 나타냄
- `ifdata -pa -pn eth0` : 지정된 인터페이스의 IPv4 주소(address)와 넷마스크(netmask)를 표시
- `ifdata -pN -pb -pm eth0` : 네트워크(Network) 주소, 브로드캐스트(broadcast) 주소 및 지정된 인터페이스의 MTU를 표시
- `ifdata` : 도움말 표시

## Links
- More information: https://manned.org/ifdata
