# podman-compose

Compose Specification 컨테이너 정의를 실행하고 관리.

## Usage
- `podman-compose ps` : 실행 중인 모든 컨테이너 나열
- `podman-compose up -d` : 로컬 `docker-compose.yml`을 사용하여 백그라운드에서 모든 컨테이너 생성 및 시작
- `podman-compose up --build` : 필요한 경우 빌드하여 모든 컨테이너 시작
- `podman-compose -f 경로/대상/파일.yaml up` : 다른 컴포즈 파일을 사용하여 모든 컨테이너 시작
- `podman-compose stop` : 실행 중인 모든 컨테이너 중지
- `podman-compose down -v` : 모든 컨테이너, 네트워크 및 볼륨 제거
- `podman-compose logs -f 컨테이너_이름` : 컨테이너의 로그를 실시간으로 팔로우 (모든 컨테이너 이름 생략)
- `podman-compose run 서비스_이름 명령` : 포트 매핑 없이 서비스에서 일회성 명령 실행

## Links
- More information: https://github.com/containers/podman-compose
