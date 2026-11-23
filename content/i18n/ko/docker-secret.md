# docker secret

Docker 스웜 비밀 관리.

## Usage
- `command | docker secret create 비밀_이름 -` : `stdin`에서 새 비밀 생성
- `docker secret create 비밀_이름 경로/대상/파일` : 파일에서 새 비밀 생성
- `docker secret ls` : 모든 비밀 나열
- `docker secret inspect --pretty 비밀_이름1 비밀_이름2 ...` : 하나 이상의 비밀에 대한 자세한 정보를 사람이 읽기 쉬운 형식으로 표시
- `docker secret rm 비밀_이름1 비밀_이름2 ...` : 하나 이상의 비밀 제거

## Links
- More information: https://docs.docker.com/reference/cli/docker/secret/
