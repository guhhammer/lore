# zlib-flate

원시 zlib 압축 및 압축 해제 프로그램. `qpdf`의 일부.

## Usage
- `zlib-flate <경로/대상/입력_파일 -compress >경로/대상/압축된.zlib` : 파일 압축
- `zlib-flate <경로/대상/압축된.zlib -uncompress >경로/대상/출력_파일` : 파일 압축 해제
- `zlib-flate <경로/대상/입력_파일 -compress=0..9 >경로/대상/압축된.zlib` : 지정된 압축 수준으로 파일 압축. 0=가장 빠름 (최악), 9=가장 느림 (최고)

## Links
- More information: https://manned.org/zlib-flate
