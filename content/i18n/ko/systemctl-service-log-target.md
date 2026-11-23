# systemctl service-log-target

서비스의 로그 출력 대상을 확인하거나 설정. D-Bus 와 통합된 서비스에서만 동작.

## Usage
- `systemctl service-log-target 서비스_이름` : 서비스의 현재 로그 대상 출력
- `systemctl service-log-target 서비스_이름 console` : 로그를 `console`로 설정 (`stderr`로 출력)
- `systemctl service-log-target 서비스_이름 journal` : 로그를 `journal`로 설정 (`systemd-journald`로 전송)
- `systemctl service-log-target 서비스_이름 syslog` : 로그를 `syslog`로 설정 (`/dev/log`로 전송)
- `systemctl service-log-target 서비스_이름 auto` : systemd가 적절한 로그 대상 자동 선택
- `systemctl service-log-target 서비스_이름 null` : 모든 로그 출력 비활성화
