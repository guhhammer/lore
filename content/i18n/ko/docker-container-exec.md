# docker container exec

이미 실행 중인 Docker 컨테이너에서 명령을 실행.

## Usage
- `docker exec -it 컨테이너_이름 /bin/bash` : 이미 실행 중인 컨테이너에서 대화형 셸 세션 시작
- `docker exec -d 컨테이너_이름 명령` : 실행 중인 컨테이너에서 백그라운드(분리 모드)로 명령 실행
- `docker exec -it -w 경로/대상/폴더 컨테이너_이름 명령` : 특정 명령을 실행할 작업 디렉토리 선택
- `docker exec -i -d 컨테이너_이름 명령` : 기존 컨테이너에서 백그라운드로 명령을 실행하되 `stdin`을 열어 둠
- `docker exec -it -e 변수_이름=값 컨테이너_이름 /bin/bash` : 실행 중인 Bash 세션에서 환경 변수를 설정
- `docker exec -u 사용자 컨테이너_이름 명령` : 특정 사용자로 명령 실행

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
