# ntpd

시스템 시계를 원격 시간 서버나 로컬 기준 시계에 동기화하는 공식 NTP(네트워크 시간 프로토콜) 데몬.

## Usage
- `sudo ntpd` : 데몬 시작
- `sudo ntpd --quit` : 시스템 시간을 원격 서버와 한 번 동기화(동기화 후 종료)
- `sudo ntpd --panicgate --quit` : "큰" 조정을 허용하여 한 번 동기화

## Links
- More information: https://manned.org/ntpd
