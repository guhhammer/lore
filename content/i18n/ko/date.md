# date

시스템 날짜 설정 및 표시.

## Usage
- `date +%c` : 기본 로컬 형식을 사용하여 현재 날짜 표시
- `date -u +%Y-%m-%dT%H:%M:%SZ` : 현재 날짜를 UTC 및 ISO 8601 형식으로 표시
- `date +%s` : 현재 날짜를 Unix 타임스탬프로 표시 (Unix epoch 이후 몇 초)
- `date -d @1473305798` : 기본 형식을 사용하여 특정 날짜 표시(Unix 타임스탬프로 표시)
- `date -d "2018-09-01 00:00" +%s -u` : 특정 날짜를 Unix 타임스탬프 형식으로 변환
- `date --rfc-3339 s` : 현재 날짜를 RFC-3339 형식으로 표시 (`YYYY-MM-DD hh:mm:ss+TZ`)
- `date 093023592021.59` : `MMDDhhmmYYYY.ss` (`YYYY` 와 `.ss`는 선택 사항) 형식을 사용해 현재 날짜를 설정
- `date +%V` : ISO 기준 현재 몇 번째 주인지 표시

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/date-invocation.html
