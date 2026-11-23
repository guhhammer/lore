# ip route

IP 라우팅 테이블 관리 하위 명령.

## Usage
- `ip r` : 라우팅 테이블 표시
- `sudo ip r a default via 게이트웨이_ip` : 게이트웨이 포워딩을 사용하여 기본 라우트 추가
- `sudo ip r a default dev eth0` : `eth0`을 사용하여 기본 라우트 추가
- `sudo ip r a 대상_ip via 게이트웨이_ip dev eth0` : 고정 라우트 추가
- `sudo ip r d 대상_ip dev eth0` : 고정 라우트 삭제
- `sudo ip r change|replace 대상_ip via 게이트웨이_ip dev eth0` : 고정 라우트 변경 또는 대체
- `ip r g 대상_ip` : 특정 IP 주소에 도달하기 위해 커널이 사용할 라우트 표시

## Links
- More information: https://manned.org/ip-route
