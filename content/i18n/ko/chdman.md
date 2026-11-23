# chdman

CHD (Compressed Hunks of Data) 이미지를 관리하고 변환. MAME 및 레트로 게임 이미지에서 일반적으로 사용됨.

## Usage
- `chdman createcd -i 경로/대상/파일.cue -o 경로/대상/파일.chd` : BIN/CUE 쌍(CD-ROM 이미지)으로부터 CHD를 생성
- `chdman createhd -i 경로/대상/디스크.img -o 경로/대상/디스크.chd` : 원시 하드 드라이브 이미지로부터 CHD를 생성
- `chdman extractcd -i 경로/대상/파일.chd -o 경로/대상/파일.cue` : (decompress) a CHD를 BIN/CUE 형식으로 추출(압축 해제)
- `chdman verify -i 경로/대상/파일.chd` : CHD 파일의 무결성을 확인
- `chdman info -i 경로/대상/파일.chd` : CHD 메타데이터 정보를 표시
- `chdman copy -i 경로/대상/오래된_파일.chd -o 경로/대상/새로운_파일.chd` : CHD 파일을 최신 포맷 버전으로 업데이트
- `chdman extracthd -i 경로/대상/디스크.chd -o 경로/대상/디스크.img` : 압축된 하드 드라이브 이미지를 편집을 위해 압축 해제된 형식으로 변환

## Links
- More information: https://docs.mamedev.org/tools/chdman.html
