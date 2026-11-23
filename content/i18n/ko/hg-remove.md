# hg remove

지정된 파일을 스테이징 영역에서 제거.

## Usage
- `hg remove 경로/대상/파일` : 파일 또는 디렉토리를 스테이징 영역에서 제거
- `hg remove -I 패턴` : 지정된 패턴과 일치하는 모든 스테이지된 파일 제거
- `hg remove -X 패턴` : 지정된 패턴과 일치하지 않는 모든 스테이지된 파일 제거
- `hg remove -S` : 하위 저장소를 재귀적으로 제거
- `hg remove -A` : 물리적으로 제거된 파일을 저장소에서 제거

## Links
- More information: https://www.mercurial-scm.org/help/commands/remove
