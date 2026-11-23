# cpio

아카이브 안팎으로 파일을 복사. cpio의 custom binary, old ASCII, new ASCII, crc, HPUX binary, HPUX old ASCII, old tar, POSIX.1 tar.와 같은 아카이브 형식을 지원함.

## Usage
- `echo "파일1 파일2 파일3" | cpio -o >archive.cpio` : 표준 입력에서 파일 이름 목록을 가져와서 cpio의 이진 형식으로 아카이브o에 추가
- `find 디렉토리/의/경로 | cpio -ov >archive.cpio` : 디렉토리의 모든 파일과 디렉토리를 복사하여 v상세모드에서 아카이브o에 추가
- `cpio -idv <archive.cpio` : 아카이브에 모든 파일을 i선택하여 필요한 경우 v상세모드로 d디렉토리를 생성

## Links
- More information: https://www.gnu.org/software/cpio/manual/cpio.html#Invoking-cpio
