# svn

Subversion 명령줄 클라이언트 도구.

## Usage
- `svn co 저장소/url` : 저장소에서 작업 복사본 체크아웃
- `svn up` : 저장소의 변경 사항을 작업 복사본에 반영
- `svn add 경로/대상/파일_또는_폴더` : 파일 및 디렉토리를 버전 관리에 추가하여 저장소에 추가될 준비. 다음 커밋에 추가됨
- `svn ci -m 커밋_로그_메시지 경로/대상/폴더` : 작업 복사본의 변경 사항을 저장소에 전송
- `svn log -vl 10` : 마지막 10개 리비전의 변경 사항을 표시하고 각 리비전에 수정된 파일 표시
- `svn h` : 도움말 표시

## Links
- More information: https://svnbook.red-bean.com/en/1.7/svn-book.html#svn.ref.svn
