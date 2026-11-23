# systemctl exit

서비스 매니저에 종료를 요청.

## Usage
- `systemctl exit --user` : 사용자 서비스 매니저 종료
- `systemctl exit 코드 --user` : 특정 종료 코드와 함께 사용자 서비스 매니저 종료
- `systemctl exit` : 컨테이너의 서비스 매니저 종료 요청 (컨테이너가 아닐 경우 `systemctl poweroff`와 동일)
