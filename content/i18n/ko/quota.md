# quota

사용자의 디스크 공간 사용량과 할당된 제한을 표시.

## Usage
- `quota -s` : 현재 사용자의 디스크 할당량을 사람이 읽기 쉬운 단위로 표시
- `quota -v` : 상세 출력 (저장 공간이 할당되지 않은 파일 시스템의 할당량도 표시)
- `quota -q` : 조용한 출력 (사용량이 할당량을 초과한 파일 시스템의 할당량만 표시)
- `quota -g` : 현재 사용자가 속한 그룹의 할당량 출력
- `sudo quota -u 사용자_명` : 다른 사용자의 디스크 할당량 표시

## Links
- More information: https://manned.org/quota
