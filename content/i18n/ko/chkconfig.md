# chkconfig

CentOS 6에서 서비스의 실행 레벨을 관리합니다.

## Usage
- `chkconfig --list` : 서비스와 실행 레벨 나열
- `chkconfig --list ntpd` : 특정 서비스의 실행 레벨 표시
- `chkconfig sshd on` : 부팅 시 서비스 활성화
- `chkconfig --level 2345 sshd on` : 실행 레벨 2, 3, 4, 5에서 부팅 시 서비스 활성화
- `chkconfig ntpd off` : 부팅 시 서비스 비활성화
- `chkconfig --level 3 ntpd off` : 실행 레벨 3에서 부팅 시 서비스 비활성화

## Links
- More information: https://manned.org/chkconfig
