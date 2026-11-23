# xz

XZ 및 LZMA 파일을 압축 또는 해제.

## Usage
- `xz 경로/대상/파일` : 파일을 xz로 압축
- `xz -d 경로/대상/파일.xz` : XZ 파일 압축 해제
- `xz -F lzma 경로/대상/파일` : 파일을 lzma로 압축
- `xz -d -F lzma 경로/대상/파일.lzma` : LZMA 파일 압축 해제
- `xz -d -c 경로/대상/파일.xz` : 파일 압축 해제 후 `stdout`에 쓰기 (`--keep` 포함)
- `xz -k 경로/대상/파일` : 파일을 압축하지만 원본을 삭제하지 않기
- `xz -0 경로/대상/파일` : 가장 빠른 압축으로 파일 압축
- `xz -9 경로/대상/파일` : 최고의 압축으로 파일 압축

## Links
- More information: https://manned.org/xz
