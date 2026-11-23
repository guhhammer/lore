# docker container ls

Docker 컨테이너 목록.

## Usage
- `docker ps` : 현재 실행 중인 Docker 컨테이너 목록
- `docker ps -a` : 모든 Docker 컨테이너 목록 (실행 중 및 중지됨)
- `docker ps -l` : 가장 최근에 생성된 컨테이너 표시 (모든 상태 포함)
- `docker ps -f "name=이름"` : 이름에 특정 문자열이 포함된 컨테이너 필터링
- `docker ps -f "ancestor=이미지:태그"` : 주어진 이미지를 조상으로 공유하는 컨테이너 필터링
- `docker ps -f "exited=코드" -a` : 종료 상태 코드로 컨테이너 필터링
- `docker ps -f "status=상태"` : 상태로 컨테이너 필터링 (created, running, removing, paused, exited, dead)
- `docker ps -f "volume=경로/대상/폴더" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : 특정 볼륨을 마운트하거나 특정 경로에 볼륨이 마운트된 컨테이너 필터링

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
