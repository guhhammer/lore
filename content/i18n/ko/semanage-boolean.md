# semanage boolean

SELinux 부울 설정을 영구적으로 관리합니다.

## Usage
- `sudo semanage boolean -l` : 모든 부울 설정 나열
- `sudo semanage boolean -l -C -n` : 사용자 정의 부울 설정을 제목 없이 나열
- `sudo semanage boolean -m -1|--on|-0|--off haproxy_connect_any` : 부울을 영구적으로 설정 또는 해제

## Links
- More information: https://manned.org/semanage-boolean
