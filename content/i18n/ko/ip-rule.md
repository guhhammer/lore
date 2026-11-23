# ip rule

IP 라우팅 정책 데이터베이스 관리.

## Usage
- `ip ru` : 라우팅 정책 표시
- `sudo ip ru a from 192.168.178.2/32` : 패킷 소스 주소를 기준으로 새 규칙 추가
- `sudo ip ru a to 192.168.178.2/32` : 패킷 목적지 주소를 기준으로 새 규칙 추가
- `sudo ip ru d from 192.168.178.2/32` : 패킷 소스 주소를 기준으로 규칙 삭제
- `sudo ip ru d to 192.168.178.2/32` : 패킷 목적지 주소를 기준으로 규칙 삭제
- `sudo ip ru f` : 삭제된 모든 규칙 플러시
- `ip ru s >경로/대상/ip_규칙들.dat` : 모든 규칙을 파일에 저장
- `sudo ip ru r <경로/대상/ip_규칙들.dat` : 파일에서 모든 규칙 복원

## Links
- More information: https://manned.org/ip-rule
