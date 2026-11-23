# blkpr

Persistent Reservations를 지원하는 블록 장치에서 예약을 등록, 예약, 해제, 선점 및 지우기.

## Usage
- `blkpr -c register -k 예약_키 경로/대상/장치` : 주어진 장치에 주어진 키로 새로운 예약을 등록 (명령)하기
- `blkpr -c reserve -k 예약_키 -t exclusive-access 경로/대상/장치` : 기존 예약의 유형을 배타적 접근으로 설정
- `blkpr -c preempt -K 이전_키 -k 새로운_키 -t write-exclusive 경로/대상/장치` : 주어진 키로 기존 예약을 선점하고 새로운 예약으로 교체
- `blkpr -c release -k 예약_키 -t 예약_유형 경로/대상/장치` : 주어진 장치에서 주어진 키와 유형으로 예약 해제
- `blkpr -c clear -k 키 경로/대상/장치` : 주어진 장치에서 모든 예약 지우기

## Links
- More information: https://manned.org/blkpr
