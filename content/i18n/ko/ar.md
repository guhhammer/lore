# ar

아카이브로부터 생성, 수정, 추출 (`.a`, `.so`, `.o`).

## Usage
- `ar x 경로/대상/파일.a` : 보관소로부터 모든 멤버를 추출하기
- `ar t 경로/대상/파일.ar` : 보관소 멤버 리스트 보여주기
- `ar r 경로/대상/파일.deb 경로/대상/debian-binary 경로/대상/control.tar.gz 경로/대상/data.tar.xz ...` : 보관소로 파일을 대체하거나 추가하기
- `ar s 경로/대상/파일.a` : object 파일 인덱스 삽입( `ranlib` 와 같은 기능입니다)
- `ar rs 경로/대상/파일.a 경로/대상/파일1.o 경로/대상/파일2.o ...` : 파일 및 첨부된 객체 파일 색인을 사용하여 보관소에 작성

## Links
- More information: https://manned.org/ar
