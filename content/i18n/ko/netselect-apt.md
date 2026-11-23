# netselect-apt

지연 시간이 가장 낮은 Debian 미러를 위한 `sources.list` 파일 생성.

## Usage
- `sudo netselect-apt` : 가장 낮은 지연 시간의 서버를 사용하여 `sources.list` 생성
- `sudo netselect-apt testing` : debian 브랜치를 지정, 기본적으로 stable이 사용됨
- `sudo netselect-apt -n` : non-free 섹션 포함
- `sudo netselect-apt -c 인도` : 미러 목록 조회를 위한 국가 지정

## Links
- More information: https://manned.org/netselect-apt
