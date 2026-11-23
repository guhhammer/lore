# hg add

지정한 파일을 Mercurial의 다음 커밋을 위한 스테이징 영역에 추가.

## Usage
- `hg add 경로/대상/파일` : 파일 또는 디렉토리를 스테이징 영역에 추가
- `hg add -I 패턴` : 지정된 패턴과 일치하는 모든 스테이징되지 않은 파일 추가
- `hg add -X 패턴` : 지정된 패턴과 일치하지 않는 모든 스테이징되지 않은 파일 추가
- `hg add -S` : 하위 저장소를 재귀적으로 추가
- `hg add -n` : 아무런 작업도 수행하지 않고 테스트 실행

## Links
- More information: https://www.mercurial-scm.org/help/commands/add
