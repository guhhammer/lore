# ioping

실시간으로 디스크 I/O 지연시간을 모니터링하는 도구.

## Usage
- `ioping .` : 기본 설정으로 현재 디렉터리의 디스크 I/O 지연 시간 확인
- `ioping -c 10 -s 1M /tmp` : `/tmp`에서 1MB 크기로 10회 요청하여 지연 시간 측정
- `ioping -R /dev/sdX` : `/dev/sdX`에서 디스크 탐색 속도 측정
- `ioping -RL /dev/sdX` : `/dev/sdX`에서 디스크 순차 읽기 속도 측정

## Links
- More information: https://github.com/koct9i/ioping
