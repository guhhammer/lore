# docker compose

다중 컨테이너 도커 어플리케이션 실행 및 관리.

## Usage
- `docker compose ps` : 실행 중인 모든 컨테이너 목록 보기
- `docker compose up -d` : 현재 디렉토리의 `docker-compose.yml` 파일을 사용해 모든 컨테이너를 백그라운드에서 생성하고 실행하기
- `docker compose up --build` : 모든 컨테이너 실행, 필요 시 재빌드
- `docker compose -p 프로젝트_이름 -f 경로/파일명 up` : 특정 구성 파일을 사용해 모든 컨테이너 실행
- `docker compose stop` : 실행 중인 모든 컨테이너 중지
- `docker compose down --rmi all -v` : 모든 컨테이너, 네트워크, 이미지, 볼륨 중지 및 삭제
- `docker compose logs -f` : 모든 컨테이너에 대한 로그 팔로우
- `docker compose logs -f 컨테이너_이름` : 특정 컨테이너에 대한 로그 팔로우

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
