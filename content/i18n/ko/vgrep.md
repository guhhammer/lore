# vgrep

사용하기 쉬운 grep용 페이지 도구.

## Usage
- `vgrep 검색_패턴` : 현재 디렉토리에서 패턴을 재귀적으로 검색하고 캐시
- `vgrep` : 캐시된 내용 표시
- `vgrep -s 4` : 기본 편집기로 캐시에서 "4번째" 일치 항목 열기
- `vgrep -s c3` : 캐시에서 각 일치 항목에 대해 "3" 줄의 컨텍스트 표시
- `vgrep -s t` : 트리 내 각 디렉토리에 대한 일치 항목 수 표시
- `vgrep -s f` : 트리 내 각 파일에 대한 일치 항목 수 표시
- `vgrep -i` : 캐시된 일치 항목과 함께 대화형 셸 시작

## Links
- More information: https://github.com/vrothberg/vgrep/blob/main/docs/vgrep.1.md
