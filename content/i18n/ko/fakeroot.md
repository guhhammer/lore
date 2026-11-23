# fakeroot

파일 조작을 위해 루트 권한을 가장하는 환경에서 명령을 실행.

## Usage
- `fakeroot` : fakeroot로 기본 셸 시작
- `fakeroot -- 명령어 명령_인자들` : fakeroot로 명령 실행
- `fakeroot -s 경로/대상/파일 -- 명령어 명령_인자들` : fakeroot로 명령을 실행하고 종료 시 환경을 파일에 저장
- `fakeroot -i 경로/대상/파일 -- 명령어 명령_인자들` : fakeroot 환경을 불러와 명령을 실행
- `fakeroot -u -- 명령어 명령_인자들` : 파일의 실제 소유권을 유지하면서 명령 실행 (루트 소유로 가장하지 않음)
- `fakeroot -h` : 도움말 표시

## Links
- More information: https://manned.org/fakeroot
