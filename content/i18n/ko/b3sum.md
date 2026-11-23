# b3sum

BLAKE3 암호화 체크섬을 계산.

## Usage
- `b3sum 경로/대상/파일1 경로/대상/파일2 ...` : 하나 이상의 파일에 대한 BLAKE3 체크섬을 계산
- `b3sum 경로/대상/파일1 경로/대상/파일2 ... >경로/대상/파일.b3` : BLAKE3 체크섬 목록을 게산하고 파일에 저장
- `명령어 | b3sum` : `stdin`에서 BLAKE3 체크섬을 계산
- `b3sum --check 경로/대상/파일.b3` : BLAKE3 합계 및 파일 이름이 포함된 파일을 읽고, 모든 파일에 일치하는 체크섬이 있는지 확인
- `b3sum --check --quiet 경로/대상/파일.b3` : 누락된 파일이 있거나 확인에 실패한 경우에만 메시지를 표시

## Links
- More information: https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum
