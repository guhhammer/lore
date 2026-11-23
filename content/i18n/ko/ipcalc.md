# ipcalc

IP 주소와 넷마스크를 기반으로 IP 정보(서브넷, 브로드캐스트, 호스트 범위 등)를 계산하는 도구.

## Usage
- `ipcalc 192.168.0.1` : IP 주소의 네트워크 정보 출력
- `ipcalc 192.168.0.1/24` : CIDR 표기법으로 네트워크 정보 출력
- `ipcalc 192.168.0.1 255.255.255.0` : 점 표시 넷마스크로 네트워크 정보 출력
- `ipcalc -b 192.168.0.1` : 비트 출력 숨김
- `ipcalc -s 크기1 크기2 크기3 ... 192.168.0.1` : 네트워크를 지정한 크기의 블록으로 분할
- `ipcalc -v` : 버전 표시

## Links
- More information: https://manned.org/ipcalc
