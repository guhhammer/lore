# systemctl enable

systemd 서비스를 활성화.

## Usage
- `systemctl enable 유닛` : 부팅 시 서비스가 실행되도록 설정
- `systemctl enable 유닛 --now` : 부팅 시 실행되도록 설정하고, 즉시 서비스 시작
- `systemctl enable 유닛 --user` : 로그인 시 실행되도록 사용자 유닛 활성화
