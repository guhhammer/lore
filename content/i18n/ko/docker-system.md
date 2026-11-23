# docker system

Docker 데이터를 관리하고 시스템 전반의 정보를 표시합니다.

## Usage
- `docker system` : 도움말 표시
- `docker system df` : docker 디스크 사용량 표시
- `docker system df -v` : 디스크 사용량에 대한 자세한 정보 표시
- `docker system prune` : 사용하지 않는 데이터 제거
- `docker system prune --filter "until=hourshminutesm"` : 지정된 시간 이상 전에 생성된 사용하지 않는 데이터 제거
- `docker system events` : docker 데몬의 실시간 이벤트 표시
- `docker system events -f 'type=container' --format '\{\{json .\}\}'` : 유효한 JSON 라인으로 스트리밍되는 컨테이너의 실시간 이벤트 표시
- `docker system info` : 시스템 전반의 정보 표시

## Links
- More information: https://docs.docker.com/reference/cli/docker/system/
