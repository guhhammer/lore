# systemctl whoami

프로세스가 속한 유닛을 출력. PID을 지정하지 않으면, `systemctl`이 실행된 현재 유닛을 표시.

## Usage
- `systemctl whoami` : 현재 셸(systemctl이 실행된 환경)이 속한 유닛 출력
- `systemctl whoami --user` : 사용자 서비스 매니저에서 현재 셸이 속한 유닛 출력 (로그인 세션에서 관리되는 서비스)
- `systemctl whoami pid` : 특정 프로세스가 속한 유닛 출력
- `systemctl whoami pid1 pid2 ...` : 여러 프로세스가 속한 유닛 출력
