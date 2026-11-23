# systemctl set-environment

하나 이상의 서비스 매니저 환경 변수를 설정.

## Usage
- `systemctl set-environment var value` : 단일 환경 변수 설정
- `systemctl set-environment var1 value1 var2 value2 ...` : 여러 환경 변수 한 번에 설정
- `systemctl set-environment var value --user` : 사용자 서비스 매니저용 환경 변수 설정
