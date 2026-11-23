# doctl databases options

각 데이터베이스 엔진에서 사용 가능한 옵션 탐색을 활성화.

## Usage
- `doctl d o 명령어 -t 액세스_토큰` : 액세스 토큰을 사용하여 `doctl databases options` 명령을 실행
- `doctl d o eng` : 사용 가능한 데이터베이스 엔진 목록을 검색
- `doctl d o r --engine pg|mysql|redis|mongodb` : 특정 데이터베이스 엔진에 사용 가능한 지역 목록을 검색
- `doctl d o s --engine pg|mysql|redis|mongodb` : 특정 데이터베이스 엔진에 사용 가능한 슬러그 목록을 검색
- `doctl d o v --engine pg|mysql|redis|mongodb` : 특정 데이터베이스 엔진에 사용 가능한 버전 목록을 검색

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/options/
