# svn changelist

파일 세트에 변경 목록을 연결.

## Usage
- `svn cl 변경목록_이름 경로/대상/파일1 경로/대상/파일2 ...` : 파일을 변경 목록에 추가하며, 변경 목록이 존재하지 않을 경우 생성
- `svn cl --remove 경로/대상/파일1 경로/대상/파일2 ...` : 파일을 변경 목록에서 제거
- `svn cl --remove -R --changelist 변경목록_이름 .` : 전체 변경 목록을 한 번에 제거
- `svn cl -R 변경목록_이름 경로/대상/폴더1 경로/대상/폴더2 ...` : 공백으로 구분된 디렉토리 목록의 내용을 변경 목록에 추가
- `svn commit --changelist 변경목록_이름` : 변경 목록 커밋

## Links
- More information: https://svnbook.red-bean.com/en/1.7/svn-book.html#svn.ref.svn.c.changelist
