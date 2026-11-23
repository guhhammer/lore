# tar

아카이브 유틸리티. 종종 `gzip` 또는 `bzip2`와 같은 압축 방법과 결합하여 사용됩니다.

## Usage
- `tar cf 경로/대상/파일.tar 경로/대상/파일1 경로/대상/파일2 ...` : 아카이브 생성 후 파일로 저장
- `tar czf 경로/대상/파일.tar.gz 경로/대상/파일1 경로/대상/파일2 ...` : gzipped 아카이브 생성 후 파일로 저장
- `tar czf 경로/대상/파일.tar.gz -C 경로/대상/폴더 .` : 디렉토리에서 상대 경로를 사용하여 gzipped (압축된) 아카이브 생성
- `tar xvf 경로/대상/원본.tar.ext` : (압축된) 아카이브 파일을 현재 디렉토리에 상세히 추출
- `tar xf 경로/대상/원본.tar.ext -C 경로/대상/폴더` : (압축된) 아카이브 파일을 대상 디렉토리에 추출
- `tar caf 경로/대상/파일.tar.xz 경로/대상/파일1 경로/대상/파일2 ...` : 파일 확장자를 사용하여 압축 프로그램을 자동으로 결정하고 압축 아카이브 생성 후 파일로 저장
- `tar tvf 경로/대상/원본.tar` : tar 파일의 내용을 상세히 목록화
- `tar xf 경로/대상/원본.tar --wildcards "*.html"` : 아카이브 파일에서 패턴과 일치하는 파일 추출

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html
