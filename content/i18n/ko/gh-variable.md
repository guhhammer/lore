# gh variable

GitHub Actions과 Dependabot 변수 관리.

## Usage
- `gh variable ls` : 현재 저장소의 변수 목록 표시
- `gh variable ls -o 조직` : 특정 조직의 변수 목록 표시
- `gh variable get 이름` : 현재 저장소의 변수 조회
- `gh variable set 이름` : 현재 저장소에 변수 설정 (값은 사용자 입력 요청)
- `gh variable set 이름 -e 환경_이름` : 현재 저장소의 배포 환경에 변수 설정
- `gh variable set 이름 -o 조직 -v all` : 모든 저장소에서 볼 수 있는 조직 변수 설정
- `gh variable set -f 경로/대상/파일.env` : dotenv 파일로부터 여러 변수 설정
- `gh variable delete 이름` : 현재 저장소의 변수 삭제

## Links
- More information: https://cli.github.com/manual/gh_variable
