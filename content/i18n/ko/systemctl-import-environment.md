# systemctl import-environment

쉘의 환경 변수를 systemd 환경으로 가져옴.

## Usage
- `systemctl import-environment 변수` : 변수 하나 가져오기
- `systemctl import-environment 변수_1 변수_2 ...` : 여러 변수 가져오기
- `systemctl import-environment 변수 --user` : 사용자 서비스용 환경 변수 가져오기
