# docker container stop

하나 이상의 실행 중인 컨테이너를 중지.

## Usage
- `docker stop 컨테이너` : docker 컨테이너 중지
- `docker stop -s 시그널 컨테이너` : 지정한 시그널을 전송하여 컨테이너 중지
- `docker stop -t 초 컨테이너` : 지정한 시간(초) 동안 대기한 후 강제로 컨테이너 종료
- `docker stop 컨테이너1 컨테이너2 ...` : 하나 이상의 컨테이너 중지
- `docker stop --help` : 도움말 표시

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stop/
