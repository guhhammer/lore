# hg status

작업 디렉토리에서 변경된 파일을 보여줍니다.

## Usage
- `hg status` : 변경된 파일의 상태 표시
- `hg status -m` : 수정된 파일만 표시
- `hg status -a` : 추가된 파일만 표시
- `hg status -r` : 제거된 파일만 표시
- `hg status -d` : 삭제되었지만 추적된 파일만 표시
- `hg status --rev 리비전` : 특정 변경 세트와 비교하여 작업 디렉토리의 변경 사항 표시
- `hg status -I 패턴` : 특정 글로브 패턴과 일치하는 파일만 표시
- `hg status -X 패턴` : 특정 글로브 패턴과 일치하지 않는 파일만 표시

## Links
- More information: https://www.mercurial-scm.org/help/commands/status
