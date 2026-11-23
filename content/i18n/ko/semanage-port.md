# semanage port

지속적인 SELinux 포트 정의를 관리합니다.

## Usage
- `sudo semanage port -l` : 모든 포트 레이블 규칙 나열
- `sudo semanage port -l -C -n` : 헤더 없이 사용자가 정의한 모든 포트 레이블 규칙 나열
- `sudo semanage port -a -t ssh_port_t -p tcp 22000` : 프로토콜-포트 쌍에 레이블을 지정하는 사용자가 정의한 규칙 추가
- `sudo semanage port -d -p udp 11940` : 프로토콜-포트 쌍을 사용하여 사용자가 정의한 규칙 삭제

## Links
- More information: https://manned.org/semanage-port
