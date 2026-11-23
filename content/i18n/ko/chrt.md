# chrt

프로세스의 실시간 속성을 조작합니다.

## Usage
- `chrt -p pid` : 프로세스의 속성 표시
- `chrt -a -p pid` : 프로세스의 모든 스레드 속성 표시
- `chrt -m` : `chrt`와 함께 사용할 수 있는 최소/최대 우선순위 값 표시
- `chrt -p 우선순위 pid` : 프로세스의 스케줄링 우선순위 설정
- `chrt --deadline|idle|batch|rr|fifo|other -p 우선순위 pid` : 프로세스의 스케줄링 정책 설정

## Links
- More information: https://manned.org/chrt
