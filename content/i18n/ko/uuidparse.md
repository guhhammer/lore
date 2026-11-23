# uuidparse

범용 고유 식별자(UUID) 파싱.

## Usage
- `uuidparse uuid1 uuid2 ...` : 지정된 UUID를 파싱하고 표 형식으로 출력
- `명령어 | uuidparse` : `stdin`에서 UUID 파싱
- `uuidparse -J uuid1 uuid2 ...` : JSON 출력 형식 사용
- `uuidparse -n uuid1 uuid2 ...` : 헤더 줄을 출력하지 않음
- `uuidparse -r uuid1 uuid2 ...` : 원시 출력 형식 사용
- `uuidparse -o UUID,VARIANT,TYPE,TIME` : 출력할 네 가지 열을 지정
- `uuidparse -h` : 도움말 표시

## Links
- More information: https://manned.org/uuidparse
