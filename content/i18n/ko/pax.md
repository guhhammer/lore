# pax

아카이빙 및 복사 유틸리티.

## Usage
- `pax -f 아카이브.tar` : 아카이브 내용 나열
- `pax -zf 아카이브.tar.gz` : `gzip` 아카이브 내용 나열
- `pax -wf 타겟.tar 경로/대상/파일1 경로/대상/파일2 ...` : 파일로부터 아카이브 생성
- `pax -w 경로/대상/파일1 경로/대상/파일2 ... >타겟.tar` : 출력 리디렉션을 사용해 파일로부터 아카이브 생성
- `pax -rf 소스.tar` : 현재 디렉토리에 아카이브 추출
- `pax -rw 경로/대상/파일1 경로/대상/폴더1 경로/대상/폴더2 ... target/` : 원본 메타데이터를 유지하면서 디렉토리에 복사; `target/`은 존재해야 함

## Links
- More information: https://manned.org/pax.1p
