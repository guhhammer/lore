# rcs

RCS(Revision Control System) 파일의 속성을 관리.

## Usage
- `rcs -u 경로/대상/파일` : 다른 사용자가 잠근 리비전의 잠금 해제 (권한 필요)
- `rcs -l리비전 경로/대상/파일` : 파일의 지정한 리비전 잠금
- `rcs -L 경로/대상/파일` : 엄격한 잠금 모드 설정 (check-in 전에 잠금 필요)
- `rcs -m리비전:"새로운_메시지" 경로/대상/파일` : 지정한 리비전의 로그 메시지 추가 또는 변경
- `rcs -o리비전 경로/대상/파일` : RCS 파일에서 지정한 리비전 삭제

## Links
- More information: https://manned.org/rcs
