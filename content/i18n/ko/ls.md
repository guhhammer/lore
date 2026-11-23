# ls

폴더의 내용물을 리스팅합니다.

## Usage
- `ls -1` : 한줄에 하나의 파일씩 나열
- `ls -a` : 숨겨진 파일을 포함해 모든 파일 나열
- `ls -F` : 모든 파일 나열, 폴더명에는 뒤에 '/'를 붙음
- `ls -la` : 파일의 모든 정보(권한, 소유자, 크기, 그리고 수정날짜)를 나열
- `ls -lh` : 파일의 모든정보를 나열, 다만 크기는 읽기쉽게 (KiB, MiB, GiB)를 사용
- `ls -lSR` : 파일의 모든 정보를 크기의 내림차순으로 정렬
- `ls -ltr` : 파일의 모든 정보를 수정날짜(오래된 순서)로 정렬
- `ls -d */` : 폴더만 나열

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.html
